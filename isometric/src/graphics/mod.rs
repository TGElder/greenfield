pub mod elements;
pub mod isometric;

pub use elements::*;

pub trait GraphicsBackend {
    fn draw_triangles(&mut self, triangles: &[Triangle]) -> usize;

    fn render(&mut self);
}
