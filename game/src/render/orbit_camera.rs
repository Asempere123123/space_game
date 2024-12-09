use bevy::input::common_conditions::input_pressed;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;

use std::f32::consts::{FRAC_PI_2, PI};

use super::{CameraMode, CameraPosition, MainCamera};

const INITIAL_CAMERA_ORBIT_DISTANCE: f64 = 10000000.0;
const CAMERA_ORBIT_SPEED: f32 = 1.0 / 32.0;
const CAMERA_ZOOM_SPEED: f64 = 1.0 / 100.0;

pub struct OrbitCameraPlugin;

impl Plugin for OrbitCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            PreUpdate,
            (
                handle_zoom,
                handle_drag
                    .run_if(input_pressed(MouseButton::Right).or_else(has_orbit_distance_changed)),
            )
                .run_if(camera_on_orbit_mode),
        );
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0., 0., 0.).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
            ..default()
        },
        OrbitDistance(INITIAL_CAMERA_ORBIT_DISTANCE),
        OrbitAngle { x: PI, y: 0. },
        MainCamera,
    ));
}

fn handle_drag(
    mut query: Query<(&mut Transform, &mut OrbitAngle, &OrbitDistance), With<MainCamera>>,
    mut evr_motion: EventReader<MouseMotion>,
    mut camera_position: ResMut<CameraPosition>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    if buttons.pressed(MouseButton::Right) {
        // Acumular todos los movimientos de camara que han pasado
        for drag_event in evr_motion.read() {
            for (_transform, mut orbit_angle, _orbit_distance) in query.iter_mut() {
                orbit_angle.x -= drag_event.delta.x * CAMERA_ORBIT_SPEED;

                orbit_angle.y += drag_event.delta.y * CAMERA_ORBIT_SPEED;
            }
        }
    }

    // Mover la camara
    for (mut transform, mut orbit_angle, orbit_distance) in query.iter_mut() {
        orbit_angle.y = orbit_angle.y.clamp(-FRAC_PI_2 + 0.1, FRAC_PI_2 - 0.1);

        camera_position.x = orbit_distance.0 * (orbit_angle.y.cos() * orbit_angle.x.sin()) as f64;
        camera_position.y = orbit_distance.0 * orbit_angle.y.sin() as f64;
        camera_position.z = orbit_distance.0 * (orbit_angle.y.cos() * orbit_angle.x.cos()) as f64;

        // Now broken, needs to be done manualy since camera is always at origin
        transform.look_to(
            Vec3::ZERO
                - Vec3::new(
                    camera_position.x as f32,
                    camera_position.y as f32,
                    camera_position.z as f32,
                ),
            Vec3::Y,
        );
    }
}

fn handle_zoom(
    mut query: Query<&mut OrbitDistance, With<MainCamera>>,
    mut evr_scroll: EventReader<MouseWheel>,
) {
    for scroll_event in evr_scroll.read() {
        for mut orbit_distance in query.iter_mut() {
            orbit_distance.0 += CAMERA_ZOOM_SPEED * orbit_distance.0 * (-scroll_event.y as f64);
        }
    }
}

#[derive(Component)]
struct OrbitDistance(f64);

fn has_orbit_distance_changed(
    query: Query<&OrbitDistance, (Changed<OrbitDistance>, With<MainCamera>)>,
) -> bool {
    query.iter().next().is_some()
}

fn camera_on_orbit_mode(camera_mode: Res<State<CameraMode>>) -> bool {
    camera_mode.get() == &CameraMode::Orbit
}

#[derive(Component, Debug)]
struct OrbitAngle {
    x: f32,
    y: f32,
}
