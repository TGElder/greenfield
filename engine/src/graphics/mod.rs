pub mod elements;
pub mod errors;
pub mod matrices;
pub mod projection;
pub mod projections;

use commons::geometry::{Rectangle, XY, XYZ};
pub use projection::Projection;

use elements::*;

use crate::graphics::errors::{DrawError, IndexError, RenderError, ScreenshotError};

pub trait Graphics {
    fn add_triangles(&mut self, triangles: &[Triangle]) -> Result<usize, DrawError>;

    fn render(&mut self) -> Result<(), RenderError>;

    fn screenshot(&self, path: &str) -> Result<(), ScreenshotError>;

    fn add_quads(&mut self, quads: &[Quad]) -> Result<usize, DrawError> {
        let triangles = quads
            .iter()
            .flat_map(
                |Quad {
                     corners: [a, b, c, d],
                     color,
                 }| {
                    [
                        Triangle {
                            corners: [*a, *b, *d],
                            color: *color,
                        },
                        Triangle {
                            corners: [*b, *c, *d],
                            color: *color,
                        },
                    ]
                    .into_iter()
                },
            )
            .collect::<Vec<_>>();
        self.add_triangles(&triangles)
    }

    fn look_at(&mut self, world_xyz: &XYZ<f32>, screen_xy: &XY<u32>);

    fn yaw(&mut self, yaw: f32);

    fn zoom(&mut self, zoom: f32);

    fn set_viewport(&mut self, viewport: Rectangle<u32>);

    fn world_xyz_at(&mut self, screen_xy: &XY<u32>) -> Result<XYZ<f32>, IndexError>;
}
