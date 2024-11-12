use bevy::prelude::*;

/// Represents the type of reference frame for the movement.
/// Determines how the object's movement is interpreted in relation to a reference frame.
#[derive(Reflect)]
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

    /// https://es.wikipedia.org/wiki/Semieje_mayor
    semimajor_axis: Option<f64>,
    eccentricity: Option<f64>,

    mean_movement: Option<f64>,

    current_mean_anomaly: f64,
    frame: Frame,
    parent: Parent,
}

/// Represents the central object that an orbiting object revolves around.
/// This object has properties like mass and rotation period that influence the orbit.
#[derive(Reflect)]
pub struct Parent {
    mass: f64,
}

impl Parent {
    pub fn new(mass: f64) -> Self {
        Self { mass }
    }
}

mod basics;
mod plugin;
mod solver;
mod time;

pub use crate::plugin::OrbitPlugin;

/// https://es.wikipedia.org/wiki/Constante_de_gravitación_universal
const G: f64 = 6.67430e-11;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_orbit() {
        let sun = Parent::new(1.9891e30);
        let mut earth = Orbit::new_orbit(149_598_023e3, 0.017, sun);

        assert_eq!(Some(1.9913261148403696e-7), earth.mean_movement);
        // Step about a year
        earth.step(3.154e7);
    }
}
