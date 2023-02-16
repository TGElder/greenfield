pub mod elements;
pub mod errors;
pub mod matrices;
pub mod projection;
pub mod projections;

use commons::geometry::{XY, XYZ};
pub use projection::Projection;

use elements::*;

use crate::graphics::errors::{DrawError, IndexError, RenderError, ScreenshotError, InitializationError};

pub trait Graphics {
    fn add_triangles(&mut self, triangles: &[Triangle]) -> Result<usize, DrawError>;

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

    fn add_billboards(&mut self, billboards: &[Billboard]) -> Result<usize, DrawError>;

    fn load_texture(&mut self, path: &str) -> Result<usize, InitializationError>;

    fn render(&mut self) -> Result<(), RenderError>;

    fn screenshot(&self, path: &str) -> Result<(), ScreenshotError>;

    fn look_at(&mut self, world_xyz: &XYZ<f32>, screen_xy: &XY<u32>);

    fn world_xyz_at(&mut self, screen_xy: &XY<u32>) -> Result<XYZ<f32>, IndexError>;

    fn projection(&mut self) -> &mut Box<dyn Projection>;
}
