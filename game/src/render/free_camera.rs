use bevy::{input::mouse::MouseMotion, prelude::*};

use super::{CameraMode, MainCamera};

const DEFAULT_CAMERA_SPEED: f32 = 10.0e3;
const SENS: f32 = 0.02;

pub struct FreeCameraPlugin;

impl Plugin for FreeCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraSpeed(DEFAULT_CAMERA_SPEED))
            .insert_resource(CameraRotation(0.0, 0.0))
            .add_systems(FixedUpdate, update_camera.run_if(camera_on_free_mode));
    }
}

fn camera_on_free_mode(camera_mode: Res<State<CameraMode>>) -> bool {
    camera_mode.get() == &CameraMode::Free
}

fn update_camera(
    mut query: Query<&mut Transform, With<MainCamera>>,
    mut camera_speed: ResMut<CameraSpeed>,
    mut camera_rotation: ResMut<CameraRotation>,
    keys: Res<ButtonInput<KeyCode>>,
    mut evr_motion: EventReader<MouseMotion>,
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
        camera.translation += camera_forward * camera_speed.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        camera.translation += -camera_forward * camera_speed.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        camera.translation += camera_right * camera_speed.0;
    }
    if keys.pressed(KeyCode::KeyA) {
        camera.translation += -camera_right * camera_speed.0;
    }

    // Acumular todos los movimientos de camara que han pasado
    for drag_event in evr_motion.read() {
        camera_rotation.0 -= drag_event.delta.x * SENS;
        camera_rotation.1 += drag_event.delta.y * SENS;
    }

    let radial_up = camera.translation.normalize();
    let reference = if radial_up.abs().dot(Vec3::new(0.0, 1.0, 0.0)) > 0.99 {
        Vec3::new(1.0, 0.0, 0.0)
    } else {
        Vec3::new(0.0, 1.0, 0.0)
    };
    let right = reference.cross(radial_up).normalize();

    let yaw_quat = Quat::from_axis_angle(radial_up, camera_rotation.0);
    let pitch_quat = Quat::from_axis_angle(right, camera_rotation.1);
    let rotation_quat = yaw_quat * pitch_quat;

    camera.rotation = rotation_quat;
}

#[derive(Resource)]
struct CameraSpeed(f32);

#[derive(Resource)]
struct CameraRotation(f32, f32);
