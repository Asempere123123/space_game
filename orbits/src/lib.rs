/// Represents the type of reference frame for the movement.
/// Determines how the object's movement is interpreted in relation to a reference frame.
pub enum Frame {
    /// Simulates movement dynamicaly.
    Free,
    /// Simulates movement based on orbital mechanics.
    Orbit,
}

/// Represents an orbital movement within the game.
/// The object's position and movement can be updated over time, relative to its parent's position and motion.
pub struct Orbit {
    x: Option<f64>,
    y: Option<f64>,
    z: Option<f64>,
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
struct Parent {
    mass: f64,
}

mod basics;
mod solver;

/// https://es.wikipedia.org/wiki/Constante_de_gravitación_universal
const G: f64 = 6.67430e-11;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_orbit() {
        let sun = Parent {
            mass: 1.9891e30,
        };
        let earth = Orbit::new_orbit(149_598_023e3, 0.017, sun);

        assert_eq!(Some(1.9913261148403696e-7), earth.mean_movement);
    }
}
