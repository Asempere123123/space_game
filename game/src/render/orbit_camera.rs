use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use nalgebra::{Rotation3, Unit, Vector3};

use std::f32::consts::{FRAC_PI_2, PI};

use super::{CameraMode, CameraPosition, MainCamera};

const INITIAL_CAMERA_ORBIT_DISTANCE: f32 = 10000000.0;
const CAMERA_ORBIT_SPEED: f32 = 1.0 / 32.0;
const CAMERA_ZOOM_SPEED: f32 = 1.0 / 100.0;

pub struct OrbitCameraPlugin;

impl Plugin for OrbitCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraCenter::default())
            .insert_resource(CameraUp(Vec3::Y))
            .add_systems(Startup, setup)
            .add_systems(
                PreUpdate,
                (handle_zoom, handle_drag).run_if(camera_on_orbit_mode),
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
    camera_center: Res<CameraCenter>,
    camera_up: Res<CameraUp>,
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

        let mut camera_relative_position = Vector3::new(
            orbit_distance.0 * orbit_angle.y.cos() * orbit_angle.x.sin(),
            orbit_distance.0 * orbit_angle.y.sin(),
            orbit_distance.0 * orbit_angle.y.cos() * orbit_angle.x.cos(),
        );

        // Rotate so that camera_up is up instead of Y
        let rotation_axis =
            Vector3::y().cross(&Vector3::new(camera_up.0.x, camera_up.0.y, camera_up.0.z));
        if rotation_axis.norm_squared() != 0. {
            let rotation_axis = Unit::new_unchecked(rotation_axis.normalize());

            let cos_theta = Vec3::Y.dot(camera_up.0);
            let angle = cos_theta.acos();
            let rotation = Rotation3::from_axis_angle(&rotation_axis, angle);

            camera_relative_position = rotation * camera_relative_position;
        }

        camera_position.x = (camera_center.0.x + camera_relative_position.x) as f64;
        camera_position.y = (camera_center.0.y + camera_relative_position.y) as f64;
        camera_position.z = (camera_center.0.z + camera_relative_position.z) as f64;

        transform.look_to(
            camera_center.0
                - Vec3::new(
                    camera_position.x as f32,
                    camera_position.y as f32,
                    camera_position.z as f32,
                ),
            camera_up.0,
        );
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
pub struct OrbitDistance(pub f32);

fn camera_on_orbit_mode(camera_mode: Res<State<CameraMode>>) -> bool {
    camera_mode.get() == &CameraMode::Orbit
}

#[derive(Component, Debug)]
struct OrbitAngle {
    x: f32,
    y: f32,
}

#[derive(Resource, Debug, Default)]
pub struct CameraCenter(pub Vec3);

#[derive(Resource, Debug, Default)]
pub struct CameraUp(pub Vec3);
