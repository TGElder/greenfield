use nalgebra::{Matrix4, Vector4};

use crate::graphics;
use crate::graphics::matrices::isometric;

pub struct Projection {
    pitch: f32,
    yaw: f32,
    zoom: f32,
    x_to_y_ratio: f32,
    z_max: f32,
    projection: Matrix4<f32>,
    scale: Matrix4<f32>,
    translation: Matrix4<f32>,
    composite: [[f32; 4]; 4],
    inverse: Matrix4<f32>,
}

pub struct Parameters {
    pub pitch: f32,
    pub yaw: f32,
    pub zoom: f32,
    pub x_to_y_ratio: f32,
    pub z_max: f32,
}

impl Projection {
    pub fn new(parameters: Parameters) -> Projection {
        let mut out = Projection {
            pitch: parameters.pitch,
            yaw: parameters.yaw,
            zoom: parameters.zoom,
            x_to_y_ratio: parameters.x_to_y_ratio,
            z_max: parameters.z_max,
            projection: Matrix4::identity(),
            scale: Matrix4::identity(),
            translation: Matrix4::identity(),
            composite: Matrix4::identity().into(),
            inverse: Matrix4::identity(),
        };
        out.update_projection();
        out.update_scale();
        out.update_composite();
        out
    }

    fn update_projection(&mut self) {
        self.projection = isometric(&self.yaw, &self.pitch);
    }

    fn update_scale(&mut self) {
        self.scale = [
            [self.zoom, 0.0, 0.0, 0.0],
            [0.0, self.zoom * self.x_to_y_ratio, 0.0, 0.0],
            [0.0, 0.0, self.z_max, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]
        .into();
    }

    fn update_composite(&mut self) {
        let composite = self.translation * self.scale * self.projection;
        self.inverse = composite.try_inverse().unwrap_or_else(|| {
            panic!(
                "Expected invertible isometric projection matrix but got {} = {} * {} * {}",
                composite, self.translation, self.scale, self.projection
            )
        });
        self.composite = composite.into();
    }
}

impl graphics::Projection for Projection {
    fn projection(&self) -> &[[f32; 4]; 4] {
        &self.composite
    }

    fn unproject(&self, [x, y, z]: &[f32; 3]) -> [f32; 3] {
        let gl_xyz = Vector4::new(*x, *y, *z, 1.0);
        let unprojected = self.inverse * gl_xyz;
        [unprojected.x, unprojected.y, unprojected.z]
    }

    fn look_at(&mut self, world_xyz: &[f32; 3], screen_xy: &[f32; 2]) {
        let world = Vector4::new(world_xyz[0], world_xyz[1], world_xyz[2], 1.0);
        let composite: Matrix4<f32> = self.composite.into();

        let offsets = composite * world;

        self.translation[(0, 3)] += -offsets.x + screen_xy[0];
        self.translation[(1, 3)] += -offsets.y + screen_xy[1];

        self.update_composite();
    }

    fn yaw(&mut self, yaw: f32) {
        self.yaw = yaw;
        self.update_projection();
        self.update_composite();
    }

    fn zoom(&mut self, zoom: f32) {
        self.zoom = zoom;
        self.update_scale();
        self.update_composite();
    }
}
