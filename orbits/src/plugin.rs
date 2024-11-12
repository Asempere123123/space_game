use crate::{
    time::{DeltaTime, TimeSpeed},
    Orbit,
};
use bevy::prelude::*;

pub struct OrbitPlugin;

impl Plugin for OrbitPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<crate::Orbit>()
            .insert_resource(TimeSpeed::new())
            .insert_resource(DeltaTime::new())
            .add_systems(PreUpdate, crate::time::update_delta_time)
            .add_systems(Update, update_orbits);
    }
}

fn update_orbits(mut query: Query<&mut Orbit>, delta_time: Res<DeltaTime>) {
    for mut orbit in query.iter_mut() {
        orbit.step(delta_time.seconds());
    }
}
