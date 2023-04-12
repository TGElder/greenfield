use std::f32::consts::PI;

use crate::physics::get_duration;

const GRAVITY: f32 = 9.81;

pub struct Solution {
    duration: f32,
    velocity: f32,
}

pub fn solve(run: f32, rise: f32, initial_velocity: f32) -> Option<Solution> {
    let acceleration = get_acceleration(run, rise)?;
    let distance = (run.powi(2) + rise.powi(2)).sqrt();

    let duration = get_duration(acceleration, initial_velocity, distance)?;

    let velocity = initial_velocity + acceleration * duration;

    Some(Solution { duration, velocity })
}

fn get_acceleration(run: f32, rise: f32) -> Option<f32> {
    if rise == 0.0 {
        return None;
    }
    let angle = (rise / run).atan();
    let angle_ratio = angle / (PI / 2.0);
    Some(angle_ratio * GRAVITY)
}
