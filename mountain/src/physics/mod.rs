use commons::quadratic;

mod skiing;

fn get_duration(acceleration: f32, velocity: f32, distance: f32) -> Option<f32> {
    match quadratic::solve(acceleration / 2.0, velocity, -distance) {
        quadratic::Solution::RealRoots { min: duration, .. } => Some(duration),
        _ => None,
    }
}
