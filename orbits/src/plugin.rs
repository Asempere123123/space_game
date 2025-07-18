use crate::{
    Orbit, Planet,
    time::{DeltaTime, TimeSpeed},
};
use bevy::prelude::*;

pub struct OrbitPlugin;

impl Plugin for OrbitPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<crate::Orbit>()
            .insert_resource(TimeSpeed::new())
            .insert_resource(DeltaTime::new())
            .add_systems(First, crate::time::update_delta_time)
            .add_systems(PreUpdate, update_planets)
            .add_systems(Update, update_orbits);
    }
}

fn update_orbits(mut query: Query<&mut Orbit>, delta_time: Res<DeltaTime>) {
    for mut orbit in query.iter_mut() {
        orbit.step(delta_time.seconds());
    }
}

fn update_planets(query: Query<&Planet>, delta_time: Res<DeltaTime>) {
    for body in query.iter() {
        if let Some(orbit) = &mut body.0.write().unwrap().orbit {
            orbit.step(delta_time.seconds());
        }
    }
}
