pub mod elements;
pub mod errors;
pub mod matrices;
pub mod models;
pub mod projection;
pub mod projections;
pub mod transform;

use commons::color::Rgba;
use commons::geometry::{XY, XYZ};
use commons::grid::Grid;
use commons::origin_grid::OriginGrid;
use nalgebra::Matrix4;
pub use projection::Projection;

use elements::*;

use crate::graphics::errors::{
    DrawError, IndexError, InitializationError, RenderError, ScreenshotError,
};

pub trait Graphics {
    fn load_texture(&mut self, image: &Grid<Rgba<u8>>) -> Result<usize, InitializationError>;

    fn load_texture_from_file(&mut self, path: &str) -> Result<usize, InitializationError>;

    fn modify_texture(&mut self, id: &usize, image: &OriginGrid<Rgba<u8>>)
        -> Result<(), DrawError>;

    fn create_triangles(&mut self) -> Result<usize, IndexError>;

    fn create_quads(&mut self) -> Result<usize, IndexError> {
        self.create_triangles()
    }

    fn create_overlay_triangles(&mut self) -> Result<usize, IndexError>;

    fn create_instanced_triangles(&mut self) -> Result<usize, IndexError>;

    fn create_overlay_quads(&mut self) -> Result<usize, IndexError> {
        self.create_overlay_triangles()
    }

    fn create_billboards(&mut self) -> Result<usize, IndexError>;

    fn draw_triangles(&mut self, index: &usize, triangles: &[Triangle]) -> Result<(), DrawError>;

    fn draw_quads(&mut self, index: &usize, quads: &[Quad]) -> Result<(), DrawError> {
        let triangles = to_triangles(quads);
        self.draw_triangles(index, &triangles)
    }

    fn draw_instanced_triangles(
        &mut self,
        index: &usize,
        triangles: &[Triangle],
        world_matrices: &[Matrix4<f32>],
    ) -> Result<(), DrawError>;

    fn draw_instanced_quads(
        &mut self,
        index: &usize,
        quads: &[Quad],
        matrices: &[Matrix4<f32>],
    ) -> Result<(), DrawError> {
        let triangles = to_triangles(quads);
        self.draw_instanced_triangles(index, &triangles, matrices)
    }

    fn draw_overlay_triangles(
        &mut self,
        index: &usize,
        overlay_triangles: &OverlayTriangles,
    ) -> Result<(), DrawError>;

    fn draw_overlay_quads(
        &mut self,
        index: &usize,
        overlay_quads: &OverlayQuads,
    ) -> Result<(), DrawError> {
        let triangles = overlay_quads
            .quads
            .iter()
            .flat_map(|[a, b, c, d]| [[*a, *b, *d], [*b, *c, *d]].into_iter())
            .collect();

        let overlay_triangles = OverlayTriangles {
            base_texture: overlay_quads.base_texture,
            overlay_texture: overlay_quads.overlay_texture,
            triangles,
        };

        self.draw_overlay_triangles(index, &overlay_triangles)
    }

    fn draw_billboard(&mut self, index: &usize, billboard: &Billboard) -> Result<(), DrawError>;

    fn render(&mut self) -> Result<(), RenderError>;

    fn screenshot(&self, path: &str) -> Result<(), ScreenshotError>;

    fn look_at(&mut self, world_xyz: &XYZ<f32>, screen_xy: &XY<u32>);

    fn world_xyz_at(&mut self, screen_xy: &XY<u32>) -> Result<XYZ<f32>, IndexError>;

    fn projection(&mut self) -> &mut Box<dyn Projection>;
}

fn to_triangles(quads: &[Quad]) -> Vec<Triangle> {
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
    triangles
}
