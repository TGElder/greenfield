use crate::engine::Engine;
use crate::graphics::GraphicsBackend;

pub trait EventHandler {
    fn handle(
        &mut self,
        event: &Event,
        game_loop: &mut dyn Engine,
        graphics: &mut dyn GraphicsBackend,
    );
}

pub enum Event {
    Tick,
}
