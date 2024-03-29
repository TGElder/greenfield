use commons::geometry::{xyz, XYZ};
use nalgebra::{Matrix4, Vector3};

use crate::graphics::{Quad, TexturedPosition, Triangle};

pub fn triangles_from_quads<T>(quads: &[Quad<T>]) -> Vec<Triangle<T>>
where
    T: Copy,
{
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
    println!("{:?}", normal);
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

#[derive(Default)]
pub struct Transformation {
    pub translation: Option<XYZ<f32>>,
    pub yaw: Option<f32>,
    pub pitch: Option<f32>,
    pub roll: Option<f32>,
    pub scale: Option<XYZ<f32>>,
}

pub fn transformation_matrix(
    Transformation {
        translation,
        yaw,
        pitch,
        roll,
        scale,
    }: Transformation,
) -> Matrix4<f32> {
    let translation = if let Some(translation) = translation {
        [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [translation.x, translation.y, translation.z, 1.0],
        ]
        .into()
    } else {
        Matrix4::identity()
    };

    let yaw = if let Some(yaw) = yaw {
        let cos = yaw.cos();
        let sin = yaw.sin();
        [
            [cos, sin, 0.0, 0.0],
            [-sin, cos, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]
        .into()
    } else {
        Matrix4::identity()
    };

    let pitch = if let Some(pitch) = pitch {
        let cos = pitch.cos();
        let sin = pitch.sin();
        [
            [cos, 0.0, -sin, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [sin, 0.0, cos, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]
        .into()
    } else {
        Matrix4::identity()
    };

    let roll = if let Some(roll) = roll {
        let cos = roll.cos();
        let sin = roll.sin();
        [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, cos, sin, 0.0],
            [0.0, -sin, cos, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]
        .into()
    } else {
        Matrix4::identity()
    };

    let scale = if let Some(scale) = scale {
        [
            [scale.x, 0.0, 0.0, 0.0],
            [0.0, scale.y, 0.0, 0.0],
            [0.0, 0.0, scale.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]
        .into()
    } else {
        Matrix4::identity()
    };

    translation * yaw * pitch * roll * scale
}
