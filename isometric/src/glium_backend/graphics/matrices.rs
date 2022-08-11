use nalgebra::Matrix4;

use crate::graphics::matrices::{isometric, scale};

pub struct Matrices {
    pub pitch: f32,
    pub yaw: f32,
    pub projection: Matrix4<f32>,
    pub scale: Matrix4<f32>,
    pub translation: Matrix4<f32>,
    pub composite: Matrix4<f32>,
}

impl Matrices {
    pub fn new(pitch: f32, yaw: f32, zoom: f32) -> Matrices {
        let projection = isometric(&yaw, &pitch);
        let scale = scale(&zoom);
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
