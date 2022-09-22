use nalgebra::Matrix4;

use crate::graphics;
use crate::graphics::matrices::{isometric, scale};

pub struct Projection {
    _pitch: f32,
    _yaw: f32,
    _projection: Matrix4<f32>,
    _scale: Matrix4<f32>,
    _translation: Matrix4<f32>,
    composite: [[f32; 4]; 4],
}

pub struct Parameters {
    pub pitch: f32,
    pub yaw: f32,
    pub scale: f32,
}

impl Projection {
    pub fn new(parameters: Parameters) -> Projection {
        let projection = isometric(&parameters.yaw, &parameters.pitch);
        let scale = scale(&parameters.scale);
        Projection {
            _pitch: parameters.pitch,
            _yaw: parameters.yaw,
            _projection: projection,
            _scale: scale,
            _translation: Matrix4::identity(),
            composite: (projection * scale).into(),
        }
    }
}

impl graphics::Projection for Projection {
    fn projection(&self) -> &[[f32; 4]; 4] {
        &self.composite
    }
}
