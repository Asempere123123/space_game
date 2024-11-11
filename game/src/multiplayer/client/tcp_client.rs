// Messages are a u32 + Bincoded Content

use bevy::{log, prelude::*};
use crossbeam_channel::{unbounded, Receiver, Sender};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpStream, ToSocketAddrs},
    runtime::{Builder, Runtime},
    task::JoinHandle,
};

use crate::multiplayer::{ClientServerMessage, ServerClientMessage, MAX_MSG_LENGTH};

#[derive(Resource)]
pub struct TcpClient {
    runtime: Runtime,
    sender: Channel<ClientServerMessage>,
    receiver: Channel<ServerClientMessage>,

    handles: Channel<JoinHandle<()>>,
}

impl TcpClient {
    pub fn new() -> Self {
        Self {
            runtime: Builder::new_multi_thread()
                .worker_threads(2)
                .enable_all()
                .build()
                .expect("Could not build the tokio Runtime"),
            sender: Channel::new(),
            receiver: Channel::new(),

            handles: Channel::new(),
        }
    }

    pub fn connect(&mut self, addr: impl ToSocketAddrs + Send + 'static) {
        self.disconnect();

        let sender_receiver = self.sender.receiver.clone();
        let receiver_sender = self.receiver.sender.clone();

        let handles_sender = self.handles.sender.clone();
        let connection = async move {
            let stream = TcpStream::connect(addr)
                .await
                .expect("Could not connect to selected host");
            stream
                .set_nodelay(true)
                .expect("Could not set nodelay in the server");
            let (mut stream_reader, mut stream_writer) = stream.into_split();

            let reader_handle = tokio::spawn(async move {
                let mut buffer = [0u8; MAX_MSG_LENGTH];
                log::info!("Started listening to the server");

                while let Ok(length) = stream_reader.read_u32().await {
                    if let Err(e) = stream_reader
                        .read_exact(&mut buffer[..(length as usize)])
                        .await
                    {
                        log::warn!("Error reading message: {}", e);
                    } else {
                        let msg: ServerClientMessage =
                            match bincode::deserialize(&buffer[..(length as usize)]) {
                                Ok(m) => m,
                                Err(e) => {
                                    log::warn!("Error reading message: {}", e);
                                    continue;
                                }
                            };

                        receiver_sender
                            .send(msg)
                            .expect("Error sending msg received from server");
                    }
                }

                log::warn!("Disconnected from server");
            });
            handles_sender
                .send(reader_handle)
                .expect("Could not send handle");

            let writer_handle = tokio::spawn(async move {
                while let Ok(msg) = sender_receiver.recv() {
                    let msg_serialized =
                        bincode::serialize(&msg).expect("Could not serialize message");
                    stream_writer
                        .write_u32(msg_serialized.len() as u32)
                        .await
                        .expect("Connection lost");
                    stream_writer
                        .write_all(&msg_serialized)
                        .await
                        .expect("Connection lost");
                }
            });
            handles_sender
                .send(writer_handle)
                .expect("Could not send handle");
        };

        let handles_sender = self.handles.sender.clone();
        let connect_handle = self.runtime.spawn(connection);
        handles_sender
            .send(connect_handle)
            .expect("Could not send handle");
    }

    pub fn disconnect(&mut self) {
        for handle in self.handles.receiver.try_iter() {
            handle.abort();
        }
    }
}

pub fn handle_messages_to_send(
    mut messages_to_send: EventReader<ClientServerMessage>,
    client: Res<TcpClient>,
) {
    for msg in messages_to_send.read() {
        let _ = client.sender.sender.send(msg.clone());
    }
}

pub fn handle_messages_to_receive(
    mut messages_to_send: EventWriter<ServerClientMessage>,
    client: Res<TcpClient>,
) {
    for msg in client.receiver.receiver.try_iter() {
        messages_to_send.send(msg);
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
