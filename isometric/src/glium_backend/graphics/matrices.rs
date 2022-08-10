use nalgebra::Matrix4;

use crate::graphics::isometric::isometric_projection;

pub struct Matrices {
    pub pitch: f32,
    pub yaw: f32,
    pub projection: Matrix4<f32>,
    pub scale: Matrix4<f32>,
    pub translation: Matrix4<f32>,
    pub composite: Matrix4<f32>,
}

impl Matrices {
    pub fn new(pitch: f32, yaw: f32) -> Matrices {
        let projection = isometric_projection(&yaw, &pitch);
        let scale = Matrix4::new(
            1.0 / 256.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0 / 256.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0 / 256.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        );
        Matrices {
            pitch,
            yaw,
            projection,
            scale: Matrix4::identity(),
            translation: Matrix4::identity(),
            composite: projection * scale,
        }
    }
}
