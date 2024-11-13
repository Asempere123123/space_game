use bevy::prelude::*;
use std::f64::consts::PI;
use std::sync::{Arc, RwLock};

mod orbit_camera;
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
        app.add_plugins(OrbitCameraPlugin)
            .insert_state(RenderState::MapView)
            .add_systems(Startup, setup)
            .add_systems(Update, (update_planets, update_orbits));
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

    // Root planet
    let root_planet = Arc::new(RwLock::new(Body::new(15.0, None)));

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
        orbits::Orbit::new_orbit(6.0, 0.7, PI / 2.0, 0.0, 0.0, root_planet.clone(), 0.0, 0.0),
    ));
    // Spawn a planet
    let mesh = meshes.add(Sphere::default());
    let sphere_material = materials.add(StandardMaterial::from_color(Color::srgb_u8(250, 10, 20)));
    let orbit = orbits::Orbit::new_orbit(3.0, 0.7, 0.0, 0.0, 0.0, root_planet.clone(), 0.0, 0.0);
    let planet = orbits::Planet::new(4.0, Some(orbit));
    let planet_body = planet.0.clone();
    commands.spawn((
        PbrBundle {
            mesh: mesh,
            material: sphere_material,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        planet,
    ));

    // Spawn a moon
    let mesh = meshes.add(Sphere::default());
    let sphere_material = materials.add(StandardMaterial::from_color(Color::srgb_u8(12, 10, 255)));
    commands.spawn((
        PbrBundle {
            mesh: mesh,
            material: sphere_material,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        orbits::Orbit::new_orbit(
            1.5,
            0.2,
            PI / 2.0,
            0.0,
            0.0,
            planet_body.clone(),
            5000023.0,
            -100000.0,
        ),
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
