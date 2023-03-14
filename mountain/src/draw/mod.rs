mod avatar;
mod terrain;

pub use avatar::*;
use commons::geometry::{xyz, XYZ};
use engine::graphics::elements::Quad;
use nalgebra::{Matrix4, Vector4};
pub use terrain::*;

fn transform(XYZ { x, y, z }: XYZ<f32>, transformaton: &Matrix4<f32>) -> XYZ<f32> {
    let point = Vector4::new(x, y, z, 1.0);
    let transformed = transformaton * point;
    xyz(transformed.x, transformed.y, transformed.z)
}

fn transform_quad(quad: &Quad, transformation: &Matrix4<f32>) -> Quad {
    Quad {
        color: quad.color,
        corners: [
            transform(quad.corners[0], transformation),
            transform(quad.corners[1], transformation),
            transform(quad.corners[2], transformation),
            transform(quad.corners[3], transformation),
        ],
    }
}
