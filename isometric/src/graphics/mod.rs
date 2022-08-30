pub mod elements;
pub mod matrices;

pub use elements::*;

pub trait GraphicsBackend {
    fn add_primitive(&mut self, triangles: &[Triangle]) -> usize;

    fn render(&mut self);

    fn screenshot(&self, path: &str);
}
