use bevy::prelude::*;

mod orbit_camera;
use orbit_camera::OrbitCameraPlugin;

pub struct RenderPlugin;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum RenderState {
    CloseView,
    MapView,
}

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(OrbitCameraPlugin)
            .insert_state(RenderState::MapView)
            .add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Cuboid::default());
    let cube_material = materials.add(StandardMaterial::from_color(Color::srgb_u8(124, 144, 255)));
    commands.spawn((PbrBundle {
        mesh: mesh,
        material: cube_material,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    },));

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        transform: Transform::from_xyz(-8.0, 16.0, -8.0),
        ..default()
    });
}
