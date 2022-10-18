use crate::engine::Engine;
use crate::graphics::Graphics;

pub trait EventHandler {
    fn handle(&mut self, event: &Event, game_loop: &mut dyn Engine, graphics: &mut dyn Graphics);
}

pub enum Event {
    Tick,
}
