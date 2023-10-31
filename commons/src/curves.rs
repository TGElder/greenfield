use crate::geometry::{xy, XY};

pub fn flat_curve_left(
    approach: &[XY<f32>; 2],
    turn_angle: f32,
    radius: f32,
    segments: u8,
) -> Vec<XY<f32>> {
    let approach_vector = approach[1] - approach[0];
    let right_angle_vector = xy(approach_vector.y, -approach_vector.x).normalize();
    let start = approach[1];
    let curve_center = start - right_angle_vector * radius;

    let mut out = Vec::with_capacity(segments as usize);
    let mut current_angle = (start - curve_center).angle();
    let increment = turn_angle / segments as f32;

    for _ in 0..segments {
        current_angle += increment;
        let current_vector = xy(current_angle.cos(), current_angle.sin()) * radius;
        let XY { x, y } = curve_center + current_vector;
        out.push(xy(x, y));
    }
    out
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use crate::almost_eq::assert_almost_eq;

    use super::*;

    #[test]
    fn test_flat_curve_left() {
        // when
        let points = flat_curve_left(&[xy(1.0, 2.0), xy(4.0, 3.0)], PI, 2.0, 4);

        // then
        assert_almost_eq(points[0].x, 5.156_399);
        assert_almost_eq(points[0].y, 4.002_939);

        assert_almost_eq(points[1].x, 5.264_911);
        assert_almost_eq(points[1].y, 5.529_822_3);

        assert_almost_eq(points[2].x, 4.261_971_5);
        assert_almost_eq(points[2].y, 6.686_221);

        assert_almost_eq(points[3].x, 2.735_088_8);
        assert_almost_eq(points[3].y, 6.794_733);
    }
}
