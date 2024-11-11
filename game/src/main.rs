use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod multiplayer;
mod render;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(render::RenderPlugin)
        .add_plugins((multiplayer::ServerPlugin, multiplayer::ClientPlugin))
        .add_plugins(WorldInspectorPlugin::new())
        .run();
}
