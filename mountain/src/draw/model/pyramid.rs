use std::f32::consts::PI;

use commons::color::Rgb;
use commons::geometry::{xyz, XYZ};
use engine::graphics::elements::Triangle;
use nalgebra::Vector3;

pub fn model(radius: f32, base_z: f32, peak_z: f32, color: Rgb<f32>) -> [Triangle; 3] {
    let triangle = (0..3)
        .map(|i| (i as f32) / 3.0)
        .map(|f| f * 2.0 * PI)
        .map(|radians| xyz(radians.cos() * radius, radians.sin() * radius, base_z))
        .collect::<Vec<_>>();

    [
        compute_triangle([xyz(0.0, 0.0, peak_z), triangle[0], triangle[1]], color),
        compute_triangle([xyz(0.0, 0.0, peak_z), triangle[1], triangle[2]], color),
        compute_triangle([xyz(0.0, 0.0, peak_z), triangle[2], triangle[0]], color),
    ]
}

fn compute_triangle(corners: [XYZ<f32>; 3], color: Rgb<f32>) -> Triangle {
    let vectors = corners
        .iter()
        .map(|XYZ { x, y, z }| Vector3::new(*x, *y, *z))
        .collect::<Vec<_>>();
    let u = vectors[0] - vectors[1];
    let v = vectors[1] - vectors[2];
    let normal = u.cross(&v).normalize();

    Triangle {
        corners,
        normal: xyz(normal.x, normal.y, normal.z),
        color,
    }
}
