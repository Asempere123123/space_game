use std::f64::consts::FRAC_PI_2;

use bevy::{
    color::palettes::{css::*, tailwind::*},
    prelude::*,
};

mod planet;
use orbits::Orbit;
use planet::{
    create_active_planet, create_unactive_planet, update_orbit_positions, update_planet_positions,
};
use ship::{CurrentShip, ShipPlugin};

use crate::{gameplay::planet::create_unactive_invisible_planet, render::Planet};

mod ship;

#[derive(Component)]
pub struct Earth;
#[derive(Component)]
pub struct Sun;

pub struct GamePlayPlugin;

impl Plugin for GamePlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ShipPlugin)
            .add_systems(Startup, setup_planets)
            .add_systems(Update, (update_planet_positions, update_orbit_positions));
    }
}

// This could be done from a file with a parser
fn setup_planets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Root planet (Sun)
    let sun_view = Planet::from_radious_and_color(6378000000.0, YELLOW);
    let sun = create_active_planet(&mut commands, 1.989e30, None, sun_view, Some(Sun));

    // Earth
    let earth_orbit =
        orbits::Orbit::new_orbit(149.598e9, 0.0167, 0.0, 0.0, 0.0, sun.clone(), 0.0, 0.0);
    let earth_view = Planet {
        radius: 6378000.0,
        color: BLUE,
        deep_water_color: LinearRgba::new(0.0, 0.0, 0.55, 1.0),
        water_color: LinearRgba::new(0.0, 0.0, 1.0, 1.0),
        sand_color: LinearRgba::new(1.0, 0.9, 0.6, 1.0),
        grass_color: LinearRgba::new(0.0, 1.0, 0.0, 1.0),
        mountains_color: LinearRgba::new(0.5, 0.5, 0.5, 1.0),
        snow_color: LinearRgba::new(1.0, 1.0, 1.0, 1.0),
    };
    let earth = create_unactive_planet(
        &mut commands,
        5.97219e24,
        Some(earth_orbit),
        earth_view,
        Some(Earth),
    );

    let moon_orbit = orbits::Orbit::new_orbit(
        384400000.0,
        0.0549,
        0.0,
        0.08979719,
        0.0,
        earth.clone(),
        0.0,
        0.0,
    );
    let moon_view = Planet {
        radius: 6378000.0,
        color: WHITE_SMOKE,
        deep_water_color: LinearRgba::new(0.0, 0.0, 0.55, 1.0),
        water_color: LinearRgba::new(0.0, 0.0, 1.0, 1.0),
        sand_color: LinearRgba::new(1.0, 0.9, 0.6, 1.0),
        grass_color: LinearRgba::new(0.0, 1.0, 0.0, 1.0),
        mountains_color: LinearRgba::new(0.5, 0.5, 0.5, 1.0),
        snow_color: LinearRgba::new(1.0, 1.0, 1.0, 1.0),
    };
    let _moon = create_unactive_planet(
        &mut commands,
        7.34767309e22,
        Some(moon_orbit),
        moon_view,
        None::<()>,
    );

    // Mars
    let mars_orbit = orbits::Orbit::new_orbit(
        227.956e9,
        0.0935,
        0.0,
        0.032253685,
        0.0,
        sun.clone(),
        0.0,
        0.0,
    );
    let mars_view = Planet::from_radious_and_color(6378000000.0, RED);
    let mars = create_unactive_planet(
        &mut commands,
        6.4171e30,
        Some(mars_orbit),
        mars_view,
        None::<()>,
    );

    let phobos_orbit = orbits::Orbit::new_orbit(
        38440000000.0,
        0.0151,
        3.7755,
        0.01885,
        2.9533,
        mars.clone(),
        0.0,
        0.0,
    );
    let phobos_view = Planet::from_radious_and_color(2378000000.0, GRAY);
    let _phobos = create_unactive_planet(
        &mut commands,
        1.08e16,
        Some(phobos_orbit),
        phobos_view,
        None::<()>,
    );

    let deimos_orbit = orbits::Orbit::new_orbit(
        23463000000.0,
        0.00033,
        1.35624,
        0.0,
        2.9533,
        mars.clone(),
        0.0,
        0.0,
    );
    let deimos_view = Planet::from_radious_and_color(1878000000.0, YELLOW_600);
    let _deimos = create_unactive_planet(
        &mut commands,
        1.5e15,
        Some(deimos_orbit),
        deimos_view,
        None::<()>,
    );

    // Intruder
    let intruder_orbit =
        orbits::Orbit::new_orbit(200.0e9, 0.6, FRAC_PI_2, 1.4, 0.0, sun.clone(), 0.0, 0.0);
    let intruder_view = Planet::from_radious_and_color(6378000000.0, SKY_700);
    let _intruder = create_unactive_planet(
        &mut commands,
        6.4171e30,
        Some(intruder_orbit),
        intruder_view,
        None::<()>,
    );

    // Twins (Ash and Ember)
    let twin_origin_orbit = orbits::Orbit::new_orbit(
        1.082041e11,
        0.0068,
        0.963247214,
        0.0591666616,
        FRAC_PI_2 - 1.33831847,
        sun.clone(),
        0.0,
        0.0,
    );
    let twin_origin = create_unactive_invisible_planet(
        &mut commands,
        6.4171e30,
        Some(twin_origin_orbit),
        None::<()>,
    );

    let ash_orbit = orbits::Orbit::new_orbit(
        38440000000.0 / 2.,
        0.0151,
        3.7755,
        0.01885,
        2.9533,
        twin_origin.clone(),
        0.0,
        0.0,
    );
    let ash_view = Planet::from_radious_and_color(2378000000.0, AMBER_200);
    let _ash_twin = create_unactive_planet(
        &mut commands,
        1.08e16,
        Some(ash_orbit),
        ash_view,
        None::<()>,
    );

    let ember_orbit = orbits::Orbit::new_orbit(
        38440000000.0 / 2.,
        0.0151,
        6.91709265359,
        0.01885,
        2.9533,
        twin_origin.clone(),
        0.0,
        0.0,
    );
    let ember_view = Planet::from_radious_and_color(2378000000.0, ORANGE_700);
    let _ember_twin = create_unactive_planet(
        &mut commands,
        1.08e16,
        Some(ember_orbit),
        ember_view,
        None::<()>,
    );

    // Añadir la nave, en teoria no hay que hacerlo aqui pero es dnd tengo acceso a la tierra
    let mesh = meshes.add(Cuboid::new(10.0, 10.0, 20.0));
    let material = materials.add(StandardMaterial::from_color(Color::srgb_u8(128, 0, 128)));
    let orbit = Orbit::new_free(0., 0., -6379000., 0.0, 0.0, -10.0, earth.clone());
    commands.spawn((Mesh3d(mesh), MeshMaterial3d(material), CurrentShip, orbit));
}
