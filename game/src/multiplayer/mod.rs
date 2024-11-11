use bevy::prelude::*;
use serde::{Deserialize, Serialize};

const MAX_MSG_LENGTH: usize = 1024;

#[cfg(feature = "client")]
mod client;
#[cfg(feature = "server")]
mod server;

#[derive(Event, Serialize, Deserialize, Debug, Clone)]
pub enum ClientServerMessage {
    Ping,
}

#[derive(Event, Serialize, Deserialize, Debug, Clone)]
pub enum ServerClientMessage {
    Pong,
}

#[cfg(feature = "server")]
#[derive(Event, Debug)]
pub struct ServerReceiver(ClientServerMessage);

#[cfg(feature = "server")]
#[derive(Event, Debug)]
pub struct ServerSender(ServerClientMessage);

pub use client::ClientPlugin;
pub use server::ServerPlugin;
