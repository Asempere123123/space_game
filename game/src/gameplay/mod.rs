use bevy::prelude::*;

mod planet;
use planet::{
    create_active_planet, create_unactive_planet, update_orbit_positions, update_planet_positions,
};

use crate::render::Planet;

pub struct GamePlayPlugin;

impl Plugin for GamePlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_planets)
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
    let sun_view = Planet {
        radius: 6378000000.0,
    };
    let sun = create_unactive_planet(&mut commands, 1.989e30, None, sun_view);

    let earth_orbit =
        orbits::Orbit::new_orbit(149.598e9, 0.0167, 0.0, 0.0, 0.0, sun.clone(), 0.0, 0.0);
    let earth_view = Planet { radius: 6378000.0 };
    let earth = create_active_planet(&mut commands, 5.97219e24, Some(earth_orbit), earth_view);

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
    let moon_view = Planet { radius: 6378000.0 };
    let _moon = create_unactive_planet(&mut commands, 7.34767309e22, Some(moon_orbit), moon_view);

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
    let mars_view = Planet {
        radius: 6378000000.0,
    };
    let _mars = create_unactive_planet(&mut commands, 6.4171e23, Some(mars_orbit), mars_view);

    // Kevin (El Cubo)
    let mesh = meshes.add(Cuboid::new(100.0, 100.0, 100.0));
    let sphere_material = materials.add(StandardMaterial::from_color(Color::srgb_u8(128, 0, 128)));
    commands.spawn(PbrBundle {
        mesh: mesh,
        material: sphere_material,
        transform: Transform::from_xyz(0.0, 0.0, -6378000.0),
        ..default()
    });
}