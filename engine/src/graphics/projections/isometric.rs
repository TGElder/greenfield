use nalgebra::{Matrix4, Vector4};

use crate::graphics;
use crate::graphics::matrices::{isometric, scale};

pub struct Projection {
    _pitch: f32,
    _yaw: f32,
    projection: Matrix4<f32>,
    scale: Matrix4<f32>,
    translation: Matrix4<f32>,
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
        let mut out = Projection {
            _pitch: parameters.pitch,
            _yaw: parameters.yaw,
            projection,
            scale,
            translation: Matrix4::identity(),
            composite: Matrix4::identity().into(),
        };
        out.update_composite();
        out
    }

    fn update_composite(&mut self) {
        self.composite = (self.translation * self.scale * self.projection).into();
    }
}

impl graphics::Projection for Projection {
    fn projection(&self) -> &[[f32; 4]; 4] {
        &self.composite
    }

    fn look_at(&mut self, world_xyz: &[f32; 3], screen_xy: &[f32; 2]) {
        let world = Vector4::new(world_xyz[0], world_xyz[1], world_xyz[2], 1.0);
        let composite: Matrix4<f32> = self.composite.into();

        let offsets = composite * world;

        self.translation[(0, 3)] += -offsets.x + screen_xy[0];
        self.translation[(1, 3)] += -offsets.y + screen_xy[1];

        self.update_composite();
    }
}
