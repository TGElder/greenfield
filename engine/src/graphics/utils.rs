use commons::geometry::{xyz, XYZ};
use nalgebra::Vector3;

use crate::graphics::{Quad, TexturedPosition, Triangle};

pub fn triangles_from_quads(quads: &[Quad]) -> Vec<Triangle> {
    let triangles = quads
        .iter()
        .flat_map(|Quad { corners, color }| {
            let normal = quad_normal(corners);
            let [a, b, c, d] = corners;
            [
                Triangle {
                    corners: [*a, *b, *d],
                    normal,
                    color: *color,
                },
                Triangle {
                    corners: [*b, *c, *d],
                    normal,
                    color: *color,
                },
            ]
            .into_iter()
        })
        .collect::<Vec<_>>();
    triangles
}

pub fn triangle_normal(corners: &[XYZ<f32>]) -> XYZ<f32> {
    let vectors = corners
        .iter()
        .map(|XYZ { x, y, z }| Vector3::new(*x, *y, *z))
        .collect::<Vec<_>>();
    let u = vectors[0] - vectors[1];
    let v = vectors[1] - vectors[2];
    let normal = u.cross(&v).normalize();
    xyz(normal.x, normal.y, normal.z)
}

pub fn quad_normal(corners: &[XYZ<f32>]) -> XYZ<f32> {
    let vectors = corners
        .iter()
        .map(|XYZ { x, y, z }| Vector3::new(*x, *y, *z))
        .collect::<Vec<_>>();

    let u = vectors[0] - vectors[2];
    let v = vectors[1] - vectors[3];
    let normal = u.cross(&v).normalize();
    xyz(normal.x, normal.y, normal.z)
}

pub fn textured_triangles_from_textured_quads(
    quads: &[[TexturedPosition; 4]],
) -> Vec<[TexturedPosition; 3]> {
    quads
        .iter()
        .flat_map(|[a, b, c, d]| [[*a, *b, *d], [*b, *c, *d]].into_iter())
        .collect()
}
