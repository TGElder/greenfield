pub const VELOCITY_LEVELS: u8 = 8;
const MAX_VELOCITY: f32 = 10.0;

pub fn encode_velocity(v: &f32) -> Option<u8> {
    if *v < 0.0 {
        return None;
    }
    if *v > MAX_VELOCITY {
        return None;
    }
    let encoding = ((v / MAX_VELOCITY) * (VELOCITY_LEVELS as f32 - 1.0)).floor() as u8;
    Some(encoding)
}

pub fn decode_velocity(encoding: &u8) -> Option<f32> {
    if *encoding >= VELOCITY_LEVELS {
        return None;
    }
    Some((*encoding as f32 / (VELOCITY_LEVELS as f32 - 1.0)) * MAX_VELOCITY)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_zero() {
        // given
        let velocity = 0.0f32;

        // when
        let encoding = encode_velocity(&velocity);

        // then
        assert_eq!(encoding, Some(0));
    }

    #[test]
    fn encode_max() {
        // given
        let velocity = MAX_VELOCITY;

        // when
        let encoding = encode_velocity(&velocity);

        // then
        assert_eq!(encoding, Some(VELOCITY_LEVELS - 1));
    }

    #[test]
    fn encode_lt_zero() {
        // given
        let velocity = -1.0;

        // when
        let encoding = encode_velocity(&velocity);

        // then
        assert_eq!(encoding, None);
    }

    #[test]
    fn encode_gt_max() {
        // given
        let velocity = MAX_VELOCITY + 1.0;

        // when
        let encode = encode_velocity(&velocity);

        // then
        assert_eq!(encode, None);
    }

    #[test]
    fn decode_zero() {
        // given
        let encoding = 0;

        // when
        let velocity = decode_velocity(&encoding);

        // then
        assert_eq!(velocity, Some(0.0));
    }

    #[test]
    fn decode_max() {
        // given
        let encoding = VELOCITY_LEVELS - 1;

        // when
        let velocity = decode_velocity(&encoding);

        // then
        assert_eq!(velocity, Some(MAX_VELOCITY));
    }

    #[test]
    fn decode_gt_max() {
        // given
        let encoding = VELOCITY_LEVELS;

        // when
        let velocity = decode_velocity(&encoding);

        // then
        assert_eq!(velocity, None);
    }
}
