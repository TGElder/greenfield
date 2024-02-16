pub mod elements;
pub mod errors;
pub mod matrices;
pub mod models;
pub mod projection;
pub mod projections;
pub mod transform;
pub mod utils;

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

    fn create_overlay_triangles(&mut self) -> Result<usize, IndexError>;

    fn create_instanced_triangles(&mut self) -> Result<usize, IndexError>;

    fn create_billboards(&mut self) -> Result<usize, IndexError>;

    fn draw_triangles(&mut self, index: &usize, triangles: &[Triangle]) -> Result<(), DrawError>;

    fn draw_instanced_triangles(
        &mut self,
        index: &usize,
        triangles: &[Triangle],
        world_matrices: &[Matrix4<f32>],
    ) -> Result<(), DrawError>;

    fn update_instanced_triangles(
        &mut self,
        index: &usize,
        world_matrices: &[Matrix4<f32>],
    ) -> Result<(), DrawError>;

    fn draw_overlay_triangles(
        &mut self,
        index: &usize,
        overlay_triangles: &OverlayTriangles,
    ) -> Result<(), DrawError>;

    fn draw_billboard(&mut self, index: &usize, billboard: &Billboard) -> Result<(), DrawError>;

    fn render(&mut self) -> Result<(), RenderError>;

    fn screenshot(&self, path: &str) -> Result<(), ScreenshotError>;

    fn look_at(&mut self, world_xyz: &XYZ<f32>, screen_xy: &XY<u32>);

    fn world_xyz_at(&mut self, screen_xy: &XY<u32>) -> Result<XYZ<f32>, IndexError>;

    fn projection(&mut self) -> &mut Box<dyn Projection>;
}
