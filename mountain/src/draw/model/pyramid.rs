use std::f32::consts::PI;

use commons::color::Rgb;
use commons::geometry::xyz;
use engine::graphics::elements::Triangle;

pub fn model(radius: f32, base_z: f32, peak_z: f32, color: Rgb<f32>) -> [Triangle; 3] {
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
