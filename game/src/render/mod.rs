use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use free_camera::FreeCameraPlugin;
use planet::{CurrentPlanet, Planet};
use std::sync::{Arc, RwLock};

mod free_camera;
mod orbit_camera;
mod planet;
use orbit_camera::OrbitCameraPlugin;
use orbits::Body;

#[derive(Component)]
pub struct MainCamera;

pub struct RenderPlugin;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum RenderState {
    CloseView,
    MapView,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum CameraMode {
    Orbit,
    Free,
}

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<planet::material::PlanetMaterial>::default())
            .add_plugins(FreeCameraPlugin)
            .add_plugins(OrbitCameraPlugin)
            .insert_state(RenderState::MapView)
            .insert_state(CameraMode::Orbit)
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    update_planets,
                    update_orbits,
                    toggle_camera_mode.run_if(input_just_pressed(KeyCode::KeyT)),
                    planet::update_chunks,
                    planet::on_planet_load,
                    planet::on_planet_unload,
                ),
            );
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Root planet
    let root_planet = Arc::new(RwLock::new(Body::new(15.0, None)));
    commands.spawn((
        Planet { radius: 6378000.0 },
        GlobalTransform::from_xyz(0.0, 0.0, 0.0),
        InheritedVisibility::VISIBLE,
        CurrentPlanet,
    ));

    // Spawn a moon
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
            intensity: 10_000_000_000.,
            range: 10_000_000_000.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        transform: Transform::from_xyz(-7.0e8, 7.0e8, -8.0),
        ..default()
    });

    // Cube
    let mesh = meshes.add(Cuboid::new(100.0, 100.0, 100.0));
    let sphere_material = materials.add(StandardMaterial::from_color(Color::srgb_u8(128, 0, 128)));
    commands.spawn(PbrBundle {
        mesh: mesh,
        material: sphere_material,
        transform: Transform::from_xyz(0.0, 0.0, -6378000.0),
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

fn toggle_camera_mode(
    state: Res<State<CameraMode>>,
    mut next_state: ResMut<NextState<CameraMode>>,
) {
    match state.get() {
        CameraMode::Free => next_state.set(CameraMode::Orbit),
        CameraMode::Orbit => next_state.set(CameraMode::Free),
    }
}
