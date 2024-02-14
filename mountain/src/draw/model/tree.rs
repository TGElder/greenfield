use std::f32::consts::PI;

use commons::color::Rgb;
use commons::geometry::xyz;
use engine::graphics::elements::Triangle;

const GREEN: Rgb<f32> = Rgb::new(0.0, 0.373, 0.337);
const BROWN: Rgb<f32> = Rgb::new(0.447, 0.361, 0.259);

pub fn tree() -> Vec<Triangle> {
    pyramid(1.5, 7.0, 10.0, GREEN)
        .into_iter()
        .chain(pyramid(2.0, 4.0, 8.0, GREEN))
        .chain(pyramid(0.5, 0.0, 10.0, BROWN))
        .collect()
}

pub fn pyramid(radius: f32, base_z: f32, peak_z: f32, color: Rgb<f32>) -> [Triangle; 3] {
    let triangle = (0..3)
        .map(|i| (i as f32) / 3.0)
        .map(|f| f * 2.0 * PI)
        .map(|radians| xyz(radians.cos() * radius, radians.sin() * radius, base_z))
        .collect::<Vec<_>>();

    [
        Triangle {
            corners: [xyz(0.0, 0.0, peak_z), triangle[0], triangle[1]],
            color,
        },
        Triangle {
            corners: [xyz(0.0, 0.0, peak_z), triangle[1], triangle[2]],
            color,
        },
        Triangle {
            corners: [xyz(0.0, 0.0, peak_z), triangle[2], triangle[0]],
            color,
        },
    ]
}
