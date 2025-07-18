use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod gameplay;
#[cfg(feature = "online")]
mod multiplayer;
mod render;
mod ui;

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(if cfg!(target_arch = "wasm32") {
                WindowPlugin {
                    primary_window: Some(Window {
                        mode: bevy::window::WindowMode::BorderlessFullscreen(
                            MonitorSelection::Current,
                        ),
                        ..default()
                    }),
                    ..default()
                }
            } else {
                default()
            }),
    )
    .add_plugins(orbits::OrbitPlugin)
    .add_plugins(gameplay::GamePlayPlugin)
    .add_plugins(bevy_egui::EguiPlugin::default())
    .add_plugins(WorldInspectorPlugin::new())
    .add_plugins((render::RenderPlugin, ui::UiPlugin));

    #[cfg(feature = "online")]
    app.add_plugins((multiplayer::ServerPlugin, multiplayer::ClientPlugin));

    app.run();
}
