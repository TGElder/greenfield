use commons::quadratic;

pub mod skiing;

fn get_duration(acceleration: f32, velocity: f32, distance: f32) -> Option<f32> {
    if distance == 0.0 {
        return Some(0.0);
    }
    // Not quadratic
    if acceleration == 0.0 {
        if velocity == 0.0 {
            return None;
        }
        return Some(distance / velocity);
    }
    match quadratic::solve(acceleration / 2.0, velocity, -distance) {
        quadratic::Solution::RealRoots { min, max } => {
            if min >= 0.0 {
                Some(min)
            } else if max >= 0.0 {
                Some(max)
            } else {
                None
            }
        }
        quadratic::Solution::EqualRoots(root) if root >= 0.0 => Some(root),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use commons::almost_eq::assert_almost_eq;

    use super::*;

    #[test]
    fn single_root() {
        // when
        let result = get_duration(-2.0, 2.0, 1.0);

        // then
        assert_almost_eq(result.unwrap(), 1.0);
    }

    #[test]
    fn two_positive_roots() {
        // when
        let result = get_duration(1.0, -2.0, -1.0);

        // then
        assert_almost_eq(result.unwrap(), 2.0 - 2.0f32.sqrt());
    }

    #[test]
    fn two_roots_different_sign() {
        // when
        let result = get_duration(2.0, 2.0, 1.0);

        // then
        assert_almost_eq(result.unwrap(), (-2.0 + 8.0f32.sqrt()) / 2.0);
    }

    #[test]
    fn single_negative_root() {
        // when
        let result = get_duration(2.0, 2.0, -1.0);

        // then
        assert_eq!(result, None);
    }

    #[test]
    fn two_negative_roots() {
        // when
        let result = get_duration(1.0, 2.0, -1.0);

        // then
        assert_eq!(result, None);
    }

    #[test]
    fn imaginary_root() {
        // when
        let result = get_duration(-1.0, 1.0, 1.0);

        // then
        assert_eq!(result, None);
    }

    #[test]
    fn zero_distance() {
        // when
        let result = get_duration(1.0, 2.0, 0.0);

        // then
        assert_almost_eq(result.unwrap(), 0.0);
    }

    #[test]
    fn zero_distance_and_velocity() {
        // when
        let result = get_duration(1.0, 0.0, 0.0);

        // then
        assert_almost_eq(result.unwrap(), 0.0);
    }

    #[test]
    fn zero_distance_and_acceleration() {
        // when
        let result = get_duration(0.0, 2.0, 0.0);

        // then
        assert_almost_eq(result.unwrap(), 0.0);
    }

    #[test]
    fn zero_distance_and_velocity_and_acceleration() {
        // when
        let result = get_duration(0.0, 0.0, 0.0);

        // then
        assert_almost_eq(result.unwrap(), 0.0);
    }

    #[test]
    fn zero_velocity() {
        // when
        let result = get_duration(1.0, 0.0, 1.0);

        // then
        assert_almost_eq(result.unwrap(), 2f32.sqrt());
    }

    #[test]
    fn zero_velocity_and_acceleration() {
        // when
        let result = get_duration(0.0, 0.0, 1.0);

        // then
        assert_eq!(result, None);
    }

    #[test]
    fn zero_acceleration() {
        // when
        let result = get_duration(0.0, 2.0, 1.0);

        // then
        assert_almost_eq(result.unwrap(), 0.5);
    }
}
