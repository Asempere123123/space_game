use crate::{Frame, Orbit, Parent, G};

use std::f64::consts::{PI, E};

const ECCENTRIC_ANOMALY_TOLERANCE: f64 = 1e-6;
const ECCENTRIC_ANOMALY_MAX_ITERATIONS: u32 = 100;

impl Orbit {
    pub fn new_free(x: f64, y: f64, z: f64, vx: f64, vy: f64, vz: f64, parent: Parent) -> Self {
        Self {
            x: Some(x),
            y: Some(y),
            z: Some(z),
            vx: Some(vx),
            vy: Some(vy),
            vz: Some(vz),

            semimajor_axis: None,
            eccentricity: None,

            mean_movement: None,

            current_mean_anomaly: 0.0,
            frame: Frame::Free,
            parent: parent,
        }
    }

    pub fn new_orbit(semimajor_axis: f64, eccentricity: f64, parent: Parent) -> Self {
        Self {
            x: None,
            y: None,
            z: None,
            vx: None,
            vy: None,
            vz: None,

            semimajor_axis: Some(semimajor_axis),
            eccentricity: Some(eccentricity),

            mean_movement: Some(mean_movement(semimajor_axis, &parent)),

            current_mean_anomaly: 0.0,
            frame: Frame::Orbit,
            parent: parent,
        }
    }

    /// Moves the body according to the elapsed time
    pub fn step(&mut self, seconds: f64) {
        match self.frame {
            Frame::Orbit => self.step_orbit(seconds),
            Frame::Free => todo!("Make this just apply gravity"),
        }
    }

    fn step_orbit(&mut self, seconds: f64) {
        // https://es.wikipedia.org/wiki/Anomalía_media
        self.current_mean_anomaly += self.mean_movement.expect("Selected orbit mode should have mean movement defined") * seconds;
        if self.current_mean_anomaly > 2.0 * PI {
            self.current_mean_anomaly -= 2.0 * PI;
        }

        // https://es.wikipedia.org/wiki/Anomalía_excéntrica
        let eccentricity = self.eccentricity.expect("Selected orbit mode should have eccentricity defined");
        let kepler_equation = kepler_equation_zeroed(self.current_mean_anomaly, eccentricity);
        let kepler_equation_derivative = kepler_equation_zeroed_derivative(eccentricity);
        let eccentric_anomaly = crate::solver::solve_newton_raphson(kepler_equation, kepler_equation_derivative, self.current_mean_anomaly, ECCENTRIC_ANOMALY_TOLERANCE, ECCENTRIC_ANOMALY_MAX_ITERATIONS);

        // https://es.wikipedia.org/wiki/Anomalía_verdadera
        let constant = ((1.0+E)/(1.0-E)).sqrt();
        let true_anomaly = (constant * (eccentric_anomaly / 2.0).tan()).atan() * 2.0;

        // https://en.wikipedia.org/wiki/Orbital_mechanics#Ellipse_geometry
        let semimajor_axis = self.semimajor_axis.expect("Selected orbit mode should have semimajor axis defined");
        let radius = (semimajor_axis * (1.0 - eccentricity.powi(2)))/(1.0 + eccentricity * true_anomaly.cos());

        // Polar: (true_anomaly, radius)
    }
}

/// https://es.wikipedia.org/wiki/Movimiento_medio_diario
/// https://es.wikipedia.org/wiki/Leyes_de_Kepler
/// 2*PI / T
fn mean_movement(semimajor_axis: f64, parent: &Parent) -> f64 {
    ((G*parent.mass)/semimajor_axis.powi(3)).sqrt()
}

/// https://es.wikipedia.org/wiki/Ecuación_de_Kepler
/// 0 = E - e*sen(E) - M
fn kepler_equation_zeroed(mean_anomaly: f64, eccentricity: f64) -> impl Fn(f64) -> f64 {
    move |eccentric_anomaly| eccentric_anomaly - eccentricity * eccentric_anomaly.sin() - mean_anomaly
}

fn kepler_equation_zeroed_derivative(eccentricity: f64) -> impl Fn(f64) -> f64 {
    move |eccentric_anomaly| 1.0 - eccentricity * eccentric_anomaly.cos()
}
