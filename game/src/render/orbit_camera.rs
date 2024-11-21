use bevy::input::common_conditions::input_pressed;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;

use std::f32::consts::{FRAC_PI_2, PI};

const INITIAL_CAMERA_ORBIT_DISTANCE: f32 = 10000000.0;
const CAMERA_ORBIT_SPEED: f32 = 1.0 / 32.0;
const CAMERA_ZOOM_SPEED: f32 = 1.0 / 100.0;

#[derive(Component)]
pub struct MainCamera;

pub struct OrbitCameraPlugin;

impl Plugin for OrbitCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (
                handle_zoom,
                handle_drag
                    .run_if(input_pressed(MouseButton::Right).or_else(has_orbit_distance_changed)),
            ),
        );
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0., 0., -INITIAL_CAMERA_ORBIT_DISTANCE)
                .looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
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
) {
    // Acumular todos los movimientos de camara que han pasado
    for drag_event in evr_motion.read() {
        for (_transform, mut orbit_angle, _orbit_distance) in query.iter_mut() {
            orbit_angle.x -= drag_event.delta.x * CAMERA_ORBIT_SPEED;

            orbit_angle.y += drag_event.delta.y * CAMERA_ORBIT_SPEED;
        }
    }

    // Mover la camara
    for (mut transform, mut orbit_angle, orbit_distance) in query.iter_mut() {
        orbit_angle.y = orbit_angle.y.clamp(-FRAC_PI_2 + 0.1, FRAC_PI_2 - 0.1);

        let x_position = orbit_distance.0 * orbit_angle.y.cos() * orbit_angle.x.sin();
        let y_position = orbit_distance.0 * orbit_angle.y.sin();
        let z_position = orbit_distance.0 * orbit_angle.y.cos() * orbit_angle.x.cos();

        transform.translation = Vec3 {
            x: x_position,
            y: y_position,
            z: z_position,
        };

        transform.look_at(Vec3::ZERO, Vec3::Y);
    }
}

fn handle_zoom(
    mut query: Query<&mut OrbitDistance, With<MainCamera>>,
    mut evr_scroll: EventReader<MouseWheel>,
) {
    for scroll_event in evr_scroll.read() {
        for mut orbit_distance in query.iter_mut() {
            orbit_distance.0 += CAMERA_ZOOM_SPEED * orbit_distance.0 * -scroll_event.y;
        }
    }
}

#[derive(Component)]
struct OrbitDistance(f32);

fn has_orbit_distance_changed(
    query: Query<&OrbitDistance, (Changed<OrbitDistance>, With<MainCamera>)>,
) -> bool {
    query.iter().count() > 0
}

#[derive(Component, Debug)]
struct OrbitAngle {
    x: f32,
    y: f32,
}
