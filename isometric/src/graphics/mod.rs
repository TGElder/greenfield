pub mod elements;
pub mod isometric;

pub use elements::*;

pub trait GraphicsBackend {
    fn add_primitive(&mut self, triangles: &[Triangle]) -> usize;

    fn render(&mut self);
}
