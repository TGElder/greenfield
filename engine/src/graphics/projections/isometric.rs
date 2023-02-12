use nalgebra::{Matrix4, Vector4};

use crate::graphics;
use crate::graphics::matrices::isometric;

pub struct Projection {
    projection: ProjectionParameters,
    scale: ScaleParameters,
    matrices: Matrices,
    composite: [[f32; 4]; 4],
    inverse: Matrix4<f32>,
}

pub struct ProjectionParameters {
    pub pitch: f32,
    pub yaw: f32,
}

pub struct ScaleParameters {
    pub zoom: f32,
    pub x_to_y_ratio: f32,
    pub z_max: f32,
}

#[derive(Debug)]
struct Matrices {
    projection: Matrix4<f32>,
    scale: Matrix4<f32>,
    translation: Matrix4<f32>,
}

pub struct Parameters {
    pub projection: ProjectionParameters,
    pub scale: ScaleParameters,
}

impl Projection {
    pub fn new(Parameters { projection, scale }: Parameters) -> Projection {
        let mut out = Projection {
            projection,
            scale,
            matrices: Matrices::default(),
            composite: Matrix4::identity().into(),
            inverse: Matrix4::identity(),
        };
        out.update_projection();
        out.update_scale();
        out.update_composite();
        out
    }

    fn update_projection(&mut self) {
        self.matrices.projection = self.projection.matrix();
    }

    fn update_scale(&mut self) {
        self.matrices.scale = self.scale.matrix();
    }

    fn update_composite(&mut self) {
        let composite = self.matrices.composite();
        self.inverse = composite.try_inverse().unwrap_or_else(|| {
            panic!(
                "Expected invertible isometric projection matrix but got {} from {:?}",
                composite, self.matrices
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

        self.matrices.translation[(0, 3)] += -offsets.x + screen_xy[0];
        self.matrices.translation[(1, 3)] += -offsets.y + screen_xy[1];

        self.update_composite();
    }

    fn yaw(&mut self, yaw: f32) {
        self.projection.yaw = yaw;
        self.update_projection();
        self.update_composite();
    }

    fn zoom(&mut self, zoom: f32) {
        self.scale.zoom = zoom;
        self.update_scale();
        self.update_composite();
    }
}

impl ProjectionParameters {
    fn matrix(&self) -> Matrix4<f32> {
        isometric(&self.yaw, &self.pitch)
    }
}

impl ScaleParameters {
    fn matrix(&self) -> Matrix4<f32> {
        [
            [self.zoom, 0.0, 0.0, 0.0],
            [0.0, self.zoom * self.x_to_y_ratio, 0.0, 0.0],
            [0.0, 0.0, self.z_max, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]
        .into()
    }
}

impl Matrices {
    fn composite(&self) -> Matrix4<f32> {
        self.translation * self.scale * self.projection
    }
}

impl Default for Matrices {
    fn default() -> Self {
        Self {
            projection: Matrix4::identity(),
            scale: Matrix4::identity(),
            translation: Matrix4::identity(),
        }
    }
}
