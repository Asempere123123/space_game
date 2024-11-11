use bevy::prelude::*;

mod tcp_server;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<super::ServerReceiver>()
            .add_event::<super::ServerSender>()
            .insert_resource(tcp_server::TcpServer::new())
            .add_systems(
                Update,
                (
                    tcp_server::handle_new_connection,
                    tcp_server::handle_connections_to_remove,
                    tcp_server::handle_receiver_event,
                    tcp_server::handle_sender_event,
                ),
            );
    }
}
