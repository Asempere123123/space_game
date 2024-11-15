use bevy::prelude::*;

/// Represents the type of reference frame for the movement.
/// Determines how the object's movement is interpreted in relation to a reference frame.
#[derive(Reflect, PartialEq)]
pub enum Frame {
    /// Simulates movement dynamicaly.
    Free,
    /// Simulates movement based on orbital mechanics.
    Orbit,
}

/// Represents a movement within the game.
/// The object's position and movement can be updated over time, relative to its parent's position and motion.
#[derive(Component, Reflect)]
pub struct Orbit {
    x: f64,
    y: f64,
    z: f64,
    vx: Option<f64>,
    vy: Option<f64>,
    vz: Option<f64>,
    velocity: f64,

    /// https://es.wikipedia.org/wiki/Semieje_mayor
    semimajor_axis: Option<f64>,
    /// https://es.wikipedia.org/wiki/Excentricidad_orbital
    eccentricity: Option<f64>,
    /// https://es.wikipedia.org/wiki/Argumento_del_periastro
    argument_of_periapsis: Option<f64>,
    /// https://es.wikipedia.org/wiki/Inclinaci%C3%B3n_orbital
    inclination: Option<f64>,
    /// https://es.wikipedia.org/wiki/Longitud_del_nodo_ascendente
    longitude_of_ascending_node: Option<f64>,

    mean_movement: Option<f64>,

    current_mean_anomaly: f64,
    current_eccentric_anomaly: f64,
    radius: f64,
    /// How the object should behave
    frame: Frame,
    /// When did this movement start
    epoch: f64,
    #[reflect(ignore)]
    parent: std::sync::Arc<std::sync::RwLock<Body>>,
}

/// Represents the central object that an orbiting object revolves around.
/// Usualy a Star/Planet/Moon
/// This object has properties like mass and rotation period that influence the orbit.
#[derive(Reflect, Default)]
pub struct Body {
    standard_gravitational_parameter: f64,
    pub orbit: Option<Orbit>,
}

/// Wrapper component arround a body, it represents any body that does not change
#[derive(Component)]
pub struct Planet(pub std::sync::Arc<std::sync::RwLock<Body>>);

impl Body {
    pub fn new(mass: f64, orbit: Option<Orbit>) -> Self {
        Self { standard_gravitational_parameter: mass * G, orbit }
    }
}

impl Planet {
    pub fn new(mass: f64, orbit: Option<Orbit>) -> Self {
        Self(std::sync::Arc::new(std::sync::RwLock::new(Body::new(
            mass, orbit,
        ))))
    }
}

mod basics;
mod plugin;
mod solver;
mod time;

pub use crate::plugin::OrbitPlugin;
pub use crate::time::{DeltaTime, TimeSpeed};

/// https://es.wikipedia.org/wiki/Constante_de_gravitación_universal
const G: f64 = 6.67430e-11;

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;
    use std::sync::{Arc, RwLock};

    use super::*;

    #[test]
    fn create_orbit() {
        let sun = Arc::new(RwLock::new(Body::new(1.9891e30, None)));
        let mut earth = Orbit::new_orbit(149_598_023e3, 0.017, PI / 2.0, 0.0, 0.0, sun, 0.0, 0.0);

        assert_eq!(Some(1.9913261148403696e-7), earth.mean_movement);
        // Step about a year
        earth.step(3.154e7);
    }
}
