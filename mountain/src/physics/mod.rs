use commons::quadratic;

mod skiing;

fn get_duration(acceleration: f32, velocity: f32, distance: f32) -> Option<f32> {
    let quadratic::Solution::RealRoots { min, max } = quadratic::solve(acceleration / 2.0, velocity, -distance) else {return None};
    if min >= 0.0 {
        Some(min)
    } else if max >= 0.0 {
        Some(max)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {}
}
