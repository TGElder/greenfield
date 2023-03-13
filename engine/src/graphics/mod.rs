pub mod elements;
pub mod errors;
pub mod matrices;
pub mod projection;
pub mod projections;

use commons::geometry::{XY, XYZ};
pub use projection::Projection;

use elements::*;

use crate::graphics::errors::{
    DrawError, IndexError, InitializationError, RenderError, ScreenshotError,
};

pub trait Graphics {
    fn load_texture(&mut self, path: &str) -> Result<usize, InitializationError>;

    fn create_triangles(&mut self) -> Result<usize, IndexError>;

    fn draw_triangles(&mut self, index: &usize, triangles: &[Triangle]) -> Result<(), DrawError>;

    fn create_quads(&mut self) -> Result<usize, IndexError> {
        self.create_triangles()
    }

    fn draw_quads(&mut self, index: &usize, quads: &[Quad]) -> Result<(), DrawError> {
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
        self.draw_triangles(index, &triangles)
    }

    fn create_billboards(&mut self) -> Result<usize, IndexError>;

    fn draw_billboard(&mut self, index: &usize, billboard: &Billboard) -> Result<(), DrawError>;

    fn render(&mut self) -> Result<(), RenderError>;

    fn screenshot(&self, path: &str) -> Result<(), ScreenshotError>;

    fn look_at(&mut self, world_xyz: &XYZ<f32>, screen_xy: &XY<u32>);

    fn world_xyz_at(&mut self, screen_xy: &XY<u32>) -> Result<XYZ<f32>, IndexError>;

    fn projection(&mut self) -> &mut Box<dyn Projection>;
}
