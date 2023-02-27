use commons::geometry::{xy, xyz, Rectangle, XY, XYZ};
use nalgebra::{Matrix4, Vector4};

use crate::graphics;
use crate::graphics::matrices::isometric;

pub struct Projection {
    projection: ProjectionParameters,
    scale: ScaleParameters,
    matrices: Matrices,
    composite: Matrix4<f32>,
    inverse: Matrix4<f32>,
}

pub struct ProjectionParameters {
    pub pitch: f32,
    pub yaw: f32,
}

pub struct ScaleParameters {
    pub zoom: f32,
    pub z_max: f32,
    pub viewport: Rectangle<u32>,
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
            composite: Matrix4::identity(),
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
        self.composite = self.matrices.composite();
        self.inverse = self.composite.try_inverse().unwrap_or_else(|| {
            panic!(
                "Expected invertible isometric projection matrix but got {} from {:?}",
                self.composite, self.matrices
            )
        });
    }
}

impl graphics::Projection for Projection {
    fn projection(&self) -> [[f32; 4]; 4] {
        self.composite.into()
    }

    fn scale(&self) -> [[f32; 4]; 4] {
        self.matrices.scale.into()
    }

    fn unproject(&self, XYZ { x, y, z }: &XYZ<f32>) -> XYZ<f32> {
        let gl_xyz = Vector4::new(*x, *y, *z, 1.0);
        let unprojected = self.inverse * gl_xyz;
        xyz(unprojected.x, unprojected.y, unprojected.z)
    }

    fn look_at(&mut self, world: &XYZ<f32>, screen: &XY<f32>) {
        let world = Vector4::new(world.x, world.y, world.z, 1.0);

        let offsets = self.composite * world;

        self.matrices.translation[(0, 3)] += -offsets.x + screen.x;
        self.matrices.translation[(1, 3)] += -offsets.y + screen.y;

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

    fn set_viewport(&mut self, viewport: Rectangle<u32>) {
        if self.scale.viewport == viewport {
            return;
        }
        self.scale.viewport = viewport;
        self.update_scale();

        let center = self.inverse * Vector4::default();

        self.update_composite();

        self.look_at(&xyz(center[0], center[1], 0.0), &xy(0.0, 0.0))
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
            [self.zoom / self.viewport.width as f32, 0.0, 0.0, 0.0],
            [0.0, self.zoom / self.viewport.height as f32, 0.0, 0.0],
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
