pub mod elements;
pub mod errors;
pub mod matrices;
pub mod projection;
pub mod projections;

pub use projection::Projection;

use elements::*;

use crate::graphics::errors::{DrawError, RenderError, ScreenshotError};

pub trait GraphicsBackend {
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
}
