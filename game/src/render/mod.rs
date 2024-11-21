use bevy::prelude::*;
use std::sync::{Arc, RwLock};

mod orbit_camera;
mod planet;
use orbit_camera::OrbitCameraPlugin;
use orbits::Body;

pub struct RenderPlugin;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum RenderState {
    CloseView,
    MapView,
}

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<planet::material::PlanetMaterial>::default())
            .add_plugins(OrbitCameraPlugin)
            .insert_state(RenderState::MapView)
            .add_systems(Startup, (setup, planet::test_init))
            .add_systems(Update, (update_planets, update_orbits, planet::test_update));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Root planet
    let root_planet = Arc::new(RwLock::new(Body::new(15.0, None)));

    // Spawn a planet
    let mesh = meshes.add(Sphere::new(1738100.0));
    let sphere_material = materials.add(StandardMaterial::from_color(Color::srgb_u8(12, 10, 255)));
    let orbit = orbits::Orbit::new_orbit(
        384400000.0,
        0.0549,
        0.0,
        0.08979719,
        0.0,
        root_planet.clone(),
        0.0,
        0.0,
    );
    commands.spawn((
        PbrBundle {
            mesh: mesh,
            material: sphere_material,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        orbit,
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
        let (x, y, z) = orbit.absolute_position();
        transform.translation = Vec3 {
            x: x as f32,
            y: y as f32,
            z: z as f32,
        };
    }
}

fn update_planets(mut query: Query<(&orbits::Planet, &mut Transform)>) {
    for (planet, mut transform) in query.iter_mut() {
        if let Some(orbit) = &planet.0.read().unwrap().orbit {
            let (x, y, z) = orbit.absolute_position();
            transform.translation = Vec3 {
                x: x as f32,
                y: y as f32,
                z: z as f32,
            };
        }
    }
}
