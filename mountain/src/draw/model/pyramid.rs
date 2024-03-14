use std::f32::consts::PI;

use commons::geometry::{xyz, XYZ};
use engine::graphics::elements::Triangle;
use engine::graphics::utils::triangle_normal;

pub fn model(radius: f32, base_z: f32, peak_z: f32) -> Vec<Triangle<()>> {
    let triangle = (0..3)
        .map(|i| (i as f32) / 3.0)
        .map(|f| f * 2.0 * PI)
        .map(|radians| xyz(radians.cos() * radius, radians.sin() * radius, base_z))
        .collect::<Vec<_>>();

    vec![
        compute_triangle([xyz(0.0, 0.0, peak_z), triangle[0], triangle[1]]),
        compute_triangle([xyz(0.0, 0.0, peak_z), triangle[1], triangle[2]]),
        compute_triangle([xyz(0.0, 0.0, peak_z), triangle[2], triangle[0]]),
    ]
}

fn compute_triangle(corners: [XYZ<f32>; 3]) -> Triangle<()> {
    Triangle {
        corners,
        normal: triangle_normal(&corners),
        color: (),
    }
}
