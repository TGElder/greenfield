pub mod elements;
pub mod errors;
pub mod matrices;
pub mod projection;
pub mod projections;

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
                     id,
                     corners: [a, b, c, d],
                     color,
                 }| {
                    [
                        Triangle {
                            id: *id,
                            corners: [*a, *b, *d],
                            color: *color,
                        },
                        Triangle {
                            id: *id,
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

    fn look_at(&mut self, world_xyz: &[f32; 3], screen_xy: &(u32, u32)) -> Result<(), IndexError>;
}
