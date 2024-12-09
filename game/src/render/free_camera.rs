use bevy::{input::mouse::MouseMotion, prelude::*};

use super::{CameraMode, CameraPosition, MainCamera};

const DEFAULT_CAMERA_SPEED: f32 = 10.0e3;
const SENS: f32 = 0.02;

pub struct FreeCameraPlugin;

impl Plugin for FreeCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraSpeed(DEFAULT_CAMERA_SPEED))
            .add_systems(FixedUpdate, update_camera.run_if(camera_on_free_mode));
    }
}

fn camera_on_free_mode(camera_mode: Res<State<CameraMode>>) -> bool {
    camera_mode.get() == &CameraMode::Free
}

fn update_camera(
    mut query: Query<&mut Transform, With<MainCamera>>,
    mut camera_speed: ResMut<CameraSpeed>,
    keys: Res<ButtonInput<KeyCode>>,
    mut evr_motion: EventReader<MouseMotion>,
    mut camera_position: ResMut<CameraPosition>,
) {
    let mut camera = query.single_mut();
    let camera_forward = camera.forward();
    let camera_right = camera.right();

    if keys.just_pressed(KeyCode::KeyP) {
        camera_speed.0 *= 0.5;
    }
    if keys.just_pressed(KeyCode::KeyO) {
        camera_speed.0 *= 1.5;
    }

    if keys.pressed(KeyCode::KeyW) {
        camera_position.add(camera_forward * camera_speed.0);
    }
    if keys.pressed(KeyCode::KeyS) {
        camera_position.add(-camera_forward * camera_speed.0);
    }
    if keys.pressed(KeyCode::KeyD) {
        camera_position.add(camera_right * camera_speed.0);
    }
    if keys.pressed(KeyCode::KeyA) {
        camera_position.add(-camera_right * camera_speed.0);
    }

    // Acumular todos los movimientos de camara que han pasado
    let mut x_to_rotate = 0.0;
    let mut y_to_rotate = 0.0;
    let mut roll_to_rotate = 0.0;
    for drag_event in evr_motion.read() {
        x_to_rotate -= drag_event.delta.x * SENS;
        y_to_rotate -= drag_event.delta.y * SENS;
    }
    if keys.pressed(KeyCode::KeyE) {
        roll_to_rotate -= SENS;
    }
    if keys.pressed(KeyCode::KeyQ) {
        roll_to_rotate += SENS;
    }

    let pitch_quaternion = Quat::from_axis_angle(Vec3::X, y_to_rotate);
    let yaw_quaternion = Quat::from_axis_angle(Vec3::Y, x_to_rotate);
    let roll_quaternion = Quat::from_axis_angle(Vec3::Z, roll_to_rotate);

    let new_rotation = yaw_quaternion * pitch_quaternion * roll_quaternion;
    camera.rotation *= new_rotation;
}

#[derive(Resource)]
struct CameraSpeed(f32);
