use super::get_duration;

const GRAVITY: f32 = 9.81;

#[derive(Debug)]
pub struct Solution {
    pub duration: f32,
    pub velocity: f32,
}

pub fn solve(
    initial_velocity: f32,
    run: f32,
    rise: f32,
    boost: f32,
    friction: f32,
) -> Option<Solution> {
    let acceleration = get_acceleration(run, rise, friction)? + boost;
    let distance = (run.powi(2) + rise.powi(2)).sqrt();

    let duration = get_duration(acceleration, initial_velocity, distance)?;

    let velocity = initial_velocity + acceleration * duration;

    Some(Solution { duration, velocity })
}

fn get_acceleration(run: f32, rise: f32, friction: f32) -> Option<f32> {
    if run == 0.0 {
        return None;
    }
    let angle = (-rise / run).atan();
    Some(GRAVITY * angle.sin() - friction * GRAVITY * angle.cos())
}
