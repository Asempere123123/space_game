use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod gameplay;
mod multiplayer;
mod render;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(orbits::OrbitPlugin)
        .add_plugins((render::RenderPlugin, ui::UiPlugin))
        .add_plugins((multiplayer::ServerPlugin, multiplayer::ClientPlugin))
        .add_plugins(gameplay::GamePlayPlugin)
        .add_plugins(bevy_egui::EguiPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .run();
}
