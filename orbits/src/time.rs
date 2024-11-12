use bevy::prelude::*;

#[derive(Resource)]
pub struct TimeSpeed(pub f64);

impl TimeSpeed {
    pub fn new() -> Self {
        Self(1.0)
    }
}

/// DeltaTime Wrapper that takes into account the timespeed
#[derive(Resource)]
pub struct DeltaTime(f64);

impl DeltaTime {
    pub fn new() -> Self {
        Self(0.0)
    }

    pub fn seconds(&self) -> f64 {
        self.0
    }
}

pub fn update_delta_time(
    mut deltatime: ResMut<DeltaTime>,
    time_speed: Res<TimeSpeed>,
    time: Res<Time>,
) {
    deltatime.0 = time.delta_seconds_f64() * time_speed.0;
}
