use bevy::prelude::*;

mod orbit_camera;
use orbit_camera::OrbitCameraPlugin;
use orbits::Parent;

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
            .add_systems(Startup, setup)
            .add_systems(Update, update_orbits);
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

    // Spawn a planet
    let mesh = meshes.add(Sphere::default());
    let sphere_material = materials.add(StandardMaterial::from_color(Color::srgb_u8(12, 10, 255)));
    commands.spawn((
        PbrBundle {
            mesh: mesh,
            material: sphere_material,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        orbits::Orbit::new_orbit(6.0, 0.7, Parent::new(15.0)),
    ));

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

fn update_orbits(mut query: Query<(&orbits::Orbit, &mut Transform)>) {
    for (orbit, mut transform) in query.iter_mut() {
        let (x, y, z) = orbit.position();
        transform.translation = Vec3 {
            x: x as f32,
            y: y as f32,
            z: z as f32,
        };
    }
}
