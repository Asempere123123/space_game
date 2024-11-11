// Messages are a u32 + Bincoded Content

use bevy::{log, prelude::*, utils::HashMap};
use crossbeam_channel::{unbounded, Receiver, Sender};
use std::net::SocketAddr;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream, ToSocketAddrs},
    runtime::{Builder, Runtime},
    task::JoinHandle,
};

use crate::multiplayer::{ServerReceiver, ServerSender, MAX_MSG_LENGTH};

use super::super::{ClientServerMessage, ServerClientMessage};

#[derive(Resource)]
pub struct TcpServer {
    runtime: Runtime,
    new_connections: Channel<(TcpStream, SocketAddr)>,
    connections: HashMap<SocketAddr, ClientConnection>,
    connections_to_remove: Channel<SocketAddr>,

    accept_handle: Option<JoinHandle<()>>,
}

impl TcpServer {
    pub fn new() -> Self {
        Self {
            runtime: Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("Could not build tokio runtime"),
            new_connections: Channel::new(),
            connections: HashMap::new(),
            connections_to_remove: Channel::new(),

            accept_handle: None,
        }
    }

    pub fn start(&mut self, addr: impl ToSocketAddrs + Send + 'static) {
        self.stop();

        let new_connection_sender = self.new_connections.sender.clone();

        let accept = async move {
            let listener = TcpListener::bind(addr)
                .await
                .expect("Could not bind to given addr");

            loop {
                let conn = match listener.accept().await {
                    Ok(conn) => conn,
                    Err(_e) => continue,
                };

                let _ = new_connection_sender.send(conn);
            }
        };

        let accept_handle = self.runtime.spawn(accept);
        self.accept_handle = Some(accept_handle);
    }

    pub fn stop(&mut self) {
        if let Some(handle) = &self.accept_handle {
            handle.abort();
        }

        for (_addr, connection) in &mut self.connections {
            connection.send_task_handle.abort();
            connection.receive_task_handle.abort();
        }
        self.connections = HashMap::new();
    }
}

pub fn handle_connections_to_remove(mut server: ResMut<TcpServer>) {
    let mut connections_to_remove: Option<Vec<_>> = None;
    for addr in server.connections_to_remove.receiver.try_iter() {
        log::warn!("Removing connection with: {}", addr);
        if let Some(to_remove) = &mut connections_to_remove {
            to_remove.push(addr);
        } else {
            connections_to_remove = Some(vec![addr]);
        }
    }

    if let Some(to_remove) = connections_to_remove {
        to_remove.iter().for_each(|addr| {
            server.connections.remove(addr);
        });
    }
}

pub fn handle_new_connection(mut server: ResMut<TcpServer>) {
    let mut new_connections: Option<HashMap<_, _>> = None;
    for (stream, addr) in server.new_connections.receiver.try_iter() {
        stream
            .set_nodelay(true)
            .expect("Could not set nodelay on client");
        let (mut reader, mut writer) = stream.into_split();

        let connections_to_remove_sender = server.connections_to_remove.sender.clone();
        let receive_channel = Channel::new();
        let receive_channel_sender = receive_channel.sender.clone();
        let receive_task = async move {
            let mut read_buffer = [0u8; MAX_MSG_LENGTH];
            loop {
                let length = match reader.read_u32().await {
                    Ok(l) => l,
                    Err(_e) => {
                        connections_to_remove_sender
                            .send(addr)
                            .expect("Could not remove conection");
                        break;
                    }
                };

                if length > MAX_MSG_LENGTH as u32 {
                    panic!("Message too long");
                }

                match reader
                    .read_exact(&mut read_buffer[..(length as usize)])
                    .await
                {
                    Err(_e) => {
                        connections_to_remove_sender
                            .send(addr)
                            .expect("Could not remove conection");
                        break;
                    }
                    _ => (),
                };
                let message: ClientServerMessage =
                    match bincode::deserialize(&read_buffer[..(length as usize)]) {
                        Ok(m) => m,
                        Err(_e) => {
                            connections_to_remove_sender
                                .send(addr)
                                .expect("Could not remove conection");
                            break;
                        }
                    };

                match receive_channel_sender.send(message) {
                    Err(_e) => {
                        connections_to_remove_sender
                            .send(addr)
                            .expect("Could not remove conection");
                        break;
                    }
                    _ => (),
                };
            }
        };
        let receive_task_handle = server.runtime.spawn(receive_task);

        let connections_to_remove_sender = server.connections_to_remove.sender.clone();
        let send_channel = Channel::new();
        let send_channel_receiver = send_channel.receiver.clone();
        let send_task = async move {
            while let Ok(message) = send_channel_receiver.recv() {
                let message = bincode::serialize(&message).expect("Could not serialize message");

                let length = message.len() as u32;

                match writer.write_u32(length).await {
                    Err(_e) => {
                        connections_to_remove_sender
                            .send(addr)
                            .expect("Could not remove conection");
                        break;
                    }
                    _ => (),
                };
                match writer.write_all(&message).await {
                    Err(_e) => {
                        connections_to_remove_sender
                            .send(addr)
                            .expect("Could not remove conection");
                        break;
                    }
                    _ => (),
                };
            }
        };
        let send_task_handle = server.runtime.spawn(send_task);

        if let Some(connections) = &mut new_connections {
            connections.insert(
                addr,
                ClientConnection {
                    receive_channel,
                    send_channel,

                    receive_task_handle,
                    send_task_handle,
                },
            );
        } else {
            let mut connections = HashMap::new();
            connections.insert(
                addr,
                ClientConnection {
                    receive_channel,
                    send_channel,

                    receive_task_handle,
                    send_task_handle,
                },
            );
            new_connections = Some(connections);
        }
    }

    if let Some(connections) = new_connections {
        server.connections.extend(connections);
    }
}

struct ClientConnection {
    receive_channel: Channel<ClientServerMessage>,
    send_channel: Channel<ServerClientMessage>,

    receive_task_handle: JoinHandle<()>,
    send_task_handle: JoinHandle<()>,
}

pub fn handle_receiver_event(
    mut server_receiver_writer: EventWriter<ServerReceiver>,
    server: Res<TcpServer>,
) {
    server.connections.iter().for_each(|(_addr, connection)| {
        connection
            .receive_channel
            .receiver
            .try_iter()
            .for_each(|msg| {
                server_receiver_writer.send(ServerReceiver(msg));
            });
    });
}

pub fn handle_sender_event(
    mut sender_sender_reader: EventReader<ServerSender>,
    server: Res<TcpServer>,
) {
    for to_send in sender_sender_reader.read() {
        server.connections.iter().for_each(|(_addr, connection)| {
            let _ = connection.send_channel.sender.send(to_send.0.clone());
        });
    }
}

struct Channel<T> {
    sender: Sender<T>,
    receiver: Receiver<T>,
}

impl<T> Channel<T> {
    fn new() -> Self {
        let (sender, receiver) = unbounded::<T>();

        Self { sender, receiver }
    }
}
