use std::sync::{Arc, RwLock};

use crate::{Body, Frame, Orbit};

use std::f64::consts::PI;

const ECCENTRIC_ANOMALY_TOLERANCE: f64 = 1e-6;
const ECCENTRIC_ANOMALY_MAX_ITERATIONS: u32 = 100;

impl Orbit {
    pub fn new_free(
        x: f64,
        y: f64,
        z: f64,
        vx: f64,
        vy: f64,
        vz: f64,
        parent: Arc<RwLock<Body>>,
    ) -> Self {
        Self {
            x,
            y,
            z,
            vx: Some(vx),
            vy: Some(vy),
            vz: Some(vz),
            velocity: (vx.powi(2) + vy.powi(2) + vz.powi(2)).sqrt(),

            semimajor_axis: None,
            eccentricity: None,
            argument_of_periapsis: None,
            inclination: None,
            longitude_of_ascending_node: None,

            mean_movement: None,

            current_mean_anomaly: 0.0,
            current_true_anomaly: 0.0,
            frame: Frame::Free,
            epoch: 0.0,
            parent: parent,
        }
    }

    pub fn new_orbit(
        semimajor_axis: f64,
        eccentricity: f64,
        argument_of_periapsis: f64,
        inclination: f64,
        longitude_of_ascending_node: f64,
        parent: Arc<RwLock<Body>>,
        current_epoch: f64,
        starting_epoch: f64,
    ) -> Self {
        let mut orbit = Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            vx: None,
            vy: None,
            vz: None,
            velocity: 0.0,

            semimajor_axis: Some(semimajor_axis),
            eccentricity: Some(eccentricity),
            argument_of_periapsis: Some(argument_of_periapsis),
            inclination: Some(inclination),
            longitude_of_ascending_node: Some(longitude_of_ascending_node),

            mean_movement: Some(mean_movement(semimajor_axis, &parent)),

            current_mean_anomaly: 0.0,
            current_true_anomaly: 0.0,
            frame: Frame::Orbit,
            epoch: starting_epoch,
            parent: parent,
        };

        orbit.step(current_epoch - starting_epoch);
        orbit
    }

    pub fn set_free(&mut self) {
        if self.frame == Frame::Free {
            return;
        }

        todo!("This entire function");
    }

    /// Moves the body according to the elapsed time
    pub fn step(&mut self, seconds: f64) {
        match self.frame {
            Frame::Orbit => self.step_orbit(seconds),
            Frame::Free => self.step_free(seconds),
        }
    }

    /// https://en.wikipedia.org/wiki/Verlet_integration
    /// Since this method is reasonably cheap, it can be changed to use a fixed timestep integration if future
    fn step_free(&mut self, seconds: f64) {
        let vx = self.vx.as_mut().expect("Selected orbit mode should have vx defined");
        let vy = self.vy.as_mut().expect("Selected orbit mode should have vy defined");
        let vz = self.vz.as_mut().expect("Selected orbit mode should have vz defined");
        let standard_gravitational_parameter = self.parent.read().unwrap().standard_gravitational_parameter;

        let r_squared = self.x.powi(2) + self.y.powi(2) + self.z.powi(2);
        let gravitational_acceleration = standard_gravitational_parameter / r_squared;
        
        let r = r_squared.sqrt();

        let gravitational_acceleration_x = (-self.x / r) * gravitational_acceleration;
        let gravitational_acceleration_y = (-self.y / r) * gravitational_acceleration;
        let gravitational_acceleration_z = (-self.z / r) * gravitational_acceleration;

        // Update positions using the current velocities
        let seconds_squared_halved = seconds.powi(2) / 2.0;
        self.x += *vx * seconds + gravitational_acceleration_x * seconds_squared_halved;
        self.y += *vy * seconds + gravitational_acceleration_y * seconds_squared_halved;
        self.z += *vz * seconds + gravitational_acceleration_z * seconds_squared_halved;

        // Calculate the new gravitational acceleration at the updated position
        let r_squared_new = self.x.powi(2) + self.y.powi(2) + self.z.powi(2);
        let gravitational_acceleration_new = standard_gravitational_parameter / r_squared_new;

        let r_new = r_squared_new.sqrt();

        let gravitational_acceleration_x_new = (-self.x / r_new) * gravitational_acceleration_new;
        let gravitational_acceleration_y_new = (-self.y / r_new) * gravitational_acceleration_new;
        let gravitational_acceleration_z_new = (-self.z / r_new) * gravitational_acceleration_new;

        // Update velocities based on the average of the old and new accelerations
        *vx += 0.5 * (gravitational_acceleration_x + gravitational_acceleration_x_new) * seconds;
        *vy += 0.5 * (gravitational_acceleration_y + gravitational_acceleration_y_new) * seconds;
        *vz += 0.5 * (gravitational_acceleration_z + gravitational_acceleration_z_new) * seconds;
        self.velocity = (vx.powi(2) + vy.powi(2) + vz.powi(2)).sqrt()
    }

    fn step_orbit(&mut self, seconds: f64) {
        let eccentricity = self
            .eccentricity
            .expect("Selected orbit mode should have eccentricity defined");
        match eccentricity {
            0.0..1.0 => self.step_eliptical_orbit(seconds),
            1.0 => todo!("Support parabolic orbits"),
            0.0.. => todo!("Suport hiperbolic orbits"),
            _ => unreachable!("Negative eccentricity does not make physical sense"),
        }
    }

    fn step_eliptical_orbit(&mut self, seconds: f64) {
        // https://es.wikipedia.org/wiki/Anomalía_media
        self.current_mean_anomaly = (self.current_mean_anomaly
            + self
                .mean_movement
                .expect("Selected orbit mode should have mean movement defined")
                * seconds)
            % (2.0 * PI);

        // https://es.wikipedia.org/wiki/Anomalía_excéntrica
        let eccentricity = self
            .eccentricity
            .expect("Selected orbit mode should have eccentricity defined");
        let kepler_equation = kepler_equation_zeroed(self.current_mean_anomaly, eccentricity);
        let kepler_equation_derivative = kepler_equation_zeroed_derivative(eccentricity);
        let eccentric_anomaly = crate::solver::solve_newton_raphson(
            kepler_equation,
            kepler_equation_derivative,
            self.current_mean_anomaly,
            ECCENTRIC_ANOMALY_TOLERANCE,
            ECCENTRIC_ANOMALY_MAX_ITERATIONS,
        );

        // https://es.wikipedia.org/wiki/Anomalía_verdadera
        let constant = ((1.0 + eccentricity) / (1.0 - eccentricity)).sqrt();
        let mut true_anomaly = (constant * (eccentric_anomaly / 2.0).tan()).atan() * 2.0;

        // https://en.wikipedia.org/wiki/Orbital_mechanics#Ellipse_geometry
        let semimajor_axis = self
            .semimajor_axis
            .expect("Selected orbit mode should have semimajor axis defined");
        let radius = (semimajor_axis * (1.0 - eccentricity.powi(2)))
            / (1.0 + eccentricity * true_anomaly.cos());

        // Apply argument of periapsis
        true_anomaly += self
            .argument_of_periapsis
            .expect("Selected orbit mode should have argument of periapsis defined");

        // Polar: (true_anomaly, radius)
        let mut position = nalgebra::Vector3::new(
            radius * true_anomaly.cos(),
            0.0,
            radius * true_anomaly.sin(),
        );

        // Apply inclitation and longitude of ascending node by rotating the point. COULD BE CACHED
        let inclination = self
            .inclination
            .expect("Selected orbit mode should have inclination defined");
        let longitude_of_ascending_node = self
            .longitude_of_ascending_node
            .expect("Selected orbit mode should have longitude of ascending node defined");

        let rotation_longitude_of_ascending_node = nalgebra::Rotation3::from_axis_angle(
            &nalgebra::Vector3::z_axis(),
            longitude_of_ascending_node,
        );
        let rotation_inclination =
            nalgebra::Rotation3::from_axis_angle(&nalgebra::Vector3::x_axis(), inclination);

        let roation = rotation_longitude_of_ascending_node * rotation_inclination;

        position = roation * position;

        self.x = position.x;
        self.y = position.y;
        self.z = position.z;
        // https://en.wikipedia.org/wiki/Vis-viva_equation
        self.velocity = (self.parent.read().unwrap().standard_gravitational_parameter * (2.0/radius - 1.0/semimajor_axis)).sqrt();
        self.current_true_anomaly = true_anomaly;
    }

    pub fn position(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    pub fn absolute_position(&self) -> (f64, f64, f64) {
        let mut position = (self.x, self.y, self.z);
        if let Some(parent_orbit) = &self.parent.read().unwrap().orbit {
            let parent_position = parent_orbit.absolute_position();

            position.0 += parent_position.0;
            position.1 += parent_position.1;
            position.2 += parent_position.2;
        }
        position
    }
}

/// https://es.wikipedia.org/wiki/Movimiento_medio_diario
/// https://es.wikipedia.org/wiki/Leyes_de_Kepler
/// 2*PI / T
fn mean_movement(semimajor_axis: f64, parent: &Arc<RwLock<Body>>) -> f64 {
    (parent.read().unwrap().standard_gravitational_parameter / semimajor_axis.powi(3)).sqrt()
}

/// https://es.wikipedia.org/wiki/Ecuación_de_Kepler
/// 0 = E - e*sen(E) - M
fn kepler_equation_zeroed(mean_anomaly: f64, eccentricity: f64) -> impl Fn(f64) -> f64 {
    move |eccentric_anomaly| {
        eccentric_anomaly - eccentricity * eccentric_anomaly.sin() - mean_anomaly
    }
}

fn kepler_equation_zeroed_derivative(eccentricity: f64) -> impl Fn(f64) -> f64 {
    move |eccentric_anomaly| 1.0 - eccentricity * eccentric_anomaly.cos()
}
