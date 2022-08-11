use crate::graphics::GraphicsBackend;

pub trait Game {
    fn update(&mut self, graphics: &mut dyn GraphicsBackend);
}
