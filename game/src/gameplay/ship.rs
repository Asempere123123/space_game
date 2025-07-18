use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use orbits::Orbit;

use crate::render::{CameraCenter, CameraUp, MainCamera, OrbitDistance};

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(CameraMode::Map).add_systems(
            Update,
            (
                update_ship_camera_orbit.run_if(in_state(CameraMode::Close)),
                togle_camera_mode.run_if(input_just_pressed(KeyCode::KeyM)),
            )
                .chain(),
        );
    }
}

#[derive(Component)]
pub struct CurrentShip;

fn update_ship_camera_orbit(
    query: Query<&Orbit, With<CurrentShip>>,
    mut camera_center: ResMut<CameraCenter>,
    mut camera_up: ResMut<CameraUp>,
) {
    let ship_transform = query
        .single()
        .expect("Could not get single ship")
        .position();
    camera_center.0.x = ship_transform.0 as f32;
    camera_center.0.y = ship_transform.1 as f32;
    camera_center.0.z = ship_transform.2 as f32;
    camera_up.0 = camera_center.0.normalize();
}

#[derive(States, Debug, Hash, Eq, PartialEq, Clone)]
enum CameraMode {
    Map,
    Close,
}

fn togle_camera_mode(
    state: Res<State<CameraMode>>,
    mut next_state: ResMut<NextState<CameraMode>>,
    mut orbit_distance: Query<&mut OrbitDistance, With<MainCamera>>,
    mut camera_center: ResMut<CameraCenter>,
    mut camera_up: ResMut<CameraUp>,
) {
    match state.get() {
        CameraMode::Map => {
            // TODO: Make this dependant on planet size and also cache the old positions
            orbit_distance.single_mut().unwrap().0 = 50.;

            next_state.set(CameraMode::Close);
        }
        CameraMode::Close => {
            // TODO: Make this dependant on planet size and also cache the old positions
            orbit_distance.single_mut().unwrap().0 = 10000000.;
            camera_center.0 = Vec3::ZERO;
            camera_up.0 = Vec3::Y;

            next_state.set(CameraMode::Map);
        }
    }
}
