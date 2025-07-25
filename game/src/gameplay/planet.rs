use bevy::prelude::*;
use orbits::{Body, Planet as PlanetOrbit};
use std::sync::{Arc, RwLock};

use crate::render::{CameraPosition, CurrentPlanet, Planet};

pub fn create_active_planet(
    commands: &mut Commands,
    mass: f64,
    orbit: Option<orbits::Orbit>,
    planet: Planet,
    bundle: Option<impl Bundle>,
) -> Arc<RwLock<Body>> {
    let orbit = PlanetOrbit::new(mass, orbit);
    let planet_orbit_ref = orbit.0.clone();

    let mut entity_commands = commands.spawn((
        planet,
        // Will spawn at origin for one frame before position gets updated. It can be computed here from the orbit struct
        GlobalTransform::from_xyz(0.0, 0.0, 0.0),
        Transform::from_xyz(0.0, 0.0, 0.0),
        InheritedVisibility::VISIBLE,
        CurrentPlanet,
        orbit,
    ));

    if let Some(bundle) = bundle {
        entity_commands.insert(bundle);
    }

    planet_orbit_ref
}

pub fn create_unactive_planet(
    commands: &mut Commands,
    mass: f64,
    orbit: Option<orbits::Orbit>,
    planet: Planet,
    bundle: Option<impl Bundle>,
) -> Arc<RwLock<Body>> {
    let orbit = PlanetOrbit::new(mass, orbit);
    let planet_orbit_ref = orbit.0.clone();

    let mut entity_commands = commands.spawn((
        planet,
        // Will spawn at origin for one frame before position gets updated. It can be computed here from the orbit struct
        GlobalTransform::from_xyz(0.0, 0.0, 0.0),
        Transform::from_xyz(0.0, 0.0, 0.0),
        InheritedVisibility::VISIBLE,
        orbit,
        CurrentPlanet,
    ));
    entity_commands.remove::<CurrentPlanet>();

    if let Some(bundle) = bundle {
        entity_commands.insert(bundle);
    }

    planet_orbit_ref
}

pub fn update_planet_positions(
    current_planet_query: Query<&PlanetOrbit, With<CurrentPlanet>>,
    mut planets_query: Query<(&PlanetOrbit, &mut Transform)>,
    camera_position: Res<CameraPosition>,
) {
    let current_planet = current_planet_query.single();
    let (current_planet_x, current_planet_y, current_planet_z) =
        match &current_planet.unwrap().0.read().unwrap().orbit {
            Some(orbit) => orbit.absolute_position(),
            None => (0.0, 0.0, 0.0),
        };

    for (planet, mut transform) in planets_query.iter_mut() {
        if let Some(orbit) = &planet.0.read().unwrap().orbit {
            let (x, y, z) = orbit.absolute_position();
            transform.translation = Vec3 {
                x: (x - current_planet_x - camera_position.x) as f32,
                y: (y - current_planet_y - camera_position.y) as f32,
                z: (z - current_planet_z - camera_position.z) as f32,
            };
        } else {
            // Planet does not have an orbit -> Is root -> Shoud be at origin
            transform.translation = Vec3 {
                x: (-current_planet_x - camera_position.x) as f32,
                y: (-current_planet_y - camera_position.y) as f32,
                z: (-current_planet_z - camera_position.z) as f32,
            };
        }
    }
}

pub fn update_orbit_positions(
    current_planet_query: Query<&PlanetOrbit, With<CurrentPlanet>>,
    mut orbits_query: Query<(&orbits::Orbit, &mut Transform)>,
    camera_position: Res<CameraPosition>,
) {
    let current_planet = current_planet_query.single();
    let (current_planet_x, current_planet_y, current_planet_z) =
        match &current_planet.unwrap().0.read().unwrap().orbit {
            Some(orbit) => orbit.absolute_position(),
            None => (0.0, 0.0, 0.0),
        };

    for (orbit, mut transform) in orbits_query.iter_mut() {
        let (x, y, z) = orbit.absolute_position();
        transform.translation = Vec3 {
            x: (x - current_planet_x - camera_position.x) as f32,
            y: (y - current_planet_y - camera_position.y) as f32,
            z: (z - current_planet_z - camera_position.z) as f32,
        };
    }
}
