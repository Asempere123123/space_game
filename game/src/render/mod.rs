use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use free_camera::FreeCameraPlugin;
pub use planet::{CurrentPlanet, Planet};

mod free_camera;
mod orbit_camera;
mod planet;
use orbit_camera::OrbitCameraPlugin;

#[derive(Component)]
pub struct MainCamera;

pub struct RenderPlugin;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum CameraMode {
    Orbit,
    Free,
}

#[derive(Resource, Default)]
pub struct CameraPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl CameraPosition {
    pub fn add(&mut self, b: Vec3) {
        self.x += b.x as f64;
        self.y += b.y as f64;
        self.z += b.z as f64;
    }
}

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<planet::material::PlanetMaterial>::default())
            .insert_resource(CameraPosition::default())
            .add_plugins((FreeCameraPlugin, OrbitCameraPlugin))
            .insert_state(CameraMode::Orbit)
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    toggle_camera_mode.run_if(input_just_pressed(KeyCode::KeyT)),
                    planet::update_chunks,
                    planet::on_planet_load,
                    planet::on_planet_unload,
                ),
            );
    }
}

fn setup(mut commands: Commands) {
    // Spawn a light that i think doesnt actualy work
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
