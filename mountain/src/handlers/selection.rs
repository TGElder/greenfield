use crate::handlers::Handler;

struct Selection {}

impl Handler for Selection {
    fn handle(
        &mut self,
        game: &crate::Game,
        event: &engine::events::Event,
        engine: &mut dyn engine::engine::Engine,
        graphics: &mut dyn engine::graphics::Graphics,
    ) {
        todo!()
    }
}