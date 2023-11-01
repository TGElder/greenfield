use crate::geometry::{xy, XY};

pub fn approximate_curve(
    position_initial: &XY<f32>,
    angle_initial: f32,
    angle_increment: f32,
    radius: f32,
    points: u8,
) -> Vec<XY<f32>> {
    let segment_length = ((angle_increment / 2.0).sin() * radius).abs() * 2.0;

    let mut position_current = *position_initial;
    let mut angle_current = angle_initial;

    let mut out = Vec::with_capacity(points as usize);
    for _ in 0..points {
        position_current =
            position_current + (xy(angle_current.cos(), angle_current.sin()) * segment_length);
        out.push(position_current);
        angle_current += angle_increment;
    }
    out
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use crate::almost_eq::assert_almost_eq;

    use super::*;

    #[test]
    fn test_approximate_curve() {
        // when
        let points = approximate_curve(&xy(4.0, 3.0), 0.714_449_64, (2.0 * PI) / 8.0, 2.0, 4);

        // then
        assert_eq!(points.len(), 4);

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
