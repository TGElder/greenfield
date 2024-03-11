use commons::geometry::{xyz, XYZ};
use nalgebra::{Matrix4, Vector3};

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

pub fn translation_matrix(translation: XYZ<f32>) -> Matrix4<f32> {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [translation.x, translation.y, translation.z, 1.0],
    ]
    .into()
}

pub fn transformation_matrix(
    translation: XYZ<f32>,
    yaw: f32,
    pitch: f32,
    roll: f32,
    scale: XYZ<f32>,
) -> Matrix4<f32> {
    let translation = translation_matrix(translation);

    let yaw = if yaw == 0.0 {
        Matrix4::identity()
    } else {
        let cos = yaw.cos();
        let sin = yaw.sin();
        [
            [cos, sin, 0.0, 0.0],
            [-sin, cos, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]
        .into()
    };

    let pitch = if pitch == 0.0 {
        Matrix4::identity()
    } else {
        let cos = pitch.cos();
        let sin = pitch.sin();
        [
            [cos, 0.0, -sin, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [sin, 0.0, cos, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]
        .into()
    };

    let roll = if roll == 0.0 {
        Matrix4::identity()
    } else {
        let cos = roll.cos();
        let sin = roll.sin();
        [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, cos, sin, 0.0],
            [0.0, -sin, cos, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]
        .into()
    };

    let scale: Matrix4<f32> = [
        [scale.x, 0.0, 0.0, 0.0],
        [0.0, scale.y, 0.0, 0.0],
        [0.0, 0.0, scale.z, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
    .into();

    translation * yaw * pitch * roll * scale
}
