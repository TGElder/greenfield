use crate::graphics::GraphicsBackend;

pub enum State {
    Running,
    Terminated,
}

pub trait Game {
    fn update(&mut self, graphics: &mut dyn GraphicsBackend) -> State;
}
