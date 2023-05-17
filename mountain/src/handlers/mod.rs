use engine::engine::Engine;
use engine::events::Event;
use engine::graphics::Graphics;

use crate::Game;

pub mod selection;

trait Handler {
    fn handle(
        &mut self,
        game: &Game,
        event: &Event,
        engine: &mut dyn Engine,
        graphics: &mut dyn Graphics,
    );
}
