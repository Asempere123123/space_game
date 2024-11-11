use bevy::prelude::*;

mod tcp_client;

use super::{ClientServerMessage, ServerClientMessage};

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ClientServerMessage>()
            .add_event::<ServerClientMessage>()
            .insert_resource(tcp_client::TcpClient::new())
            .add_systems(
                Update,
                (
                    tcp_client::handle_messages_to_send,
                    tcp_client::handle_messages_to_receive,
                ),
            );
    }
}
