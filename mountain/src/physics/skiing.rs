use std::f32::consts::PI;

use crate::physics::get_duration;

const GRAVITY: f32 = 9.81;

pub struct Solution {
    velocity: f32,
    duration: f32,
}

// ski angle is 0 when skis are pointing downhill, PI / 2.0 when perpendicular to the slope
pub fn solve(run: f32, rise: f32, initial_velocity: f32) -> Option<Solution> {
    let acceleration = get_acceleration(run, rise)?;
    let distance = (run.powi(2) + rise.powi(2)).sqrt();

    let Some(duration) = get_duration(acceleration, initial_velocity, distance) else {return None};

    let velocity = initial_velocity + acceleration * duration;

    Some(Solution { velocity, duration })
}

fn get_acceleration(run: f32, rise: f32) -> Option<f32> {
    if rise == 0.0 {
        return None;
    }
    let angle = (rise / run).atan();
    let angle_ratio = angle / (PI / 2.0);
    Some(angle_ratio * GRAVITY)
}
