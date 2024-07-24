use crate::{
    engine::Engine,
    events::{Event, EventHandler},
    graphics::Graphics,
};

#[derive(Default)]
pub struct Handler {}

impl Handler {
    pub fn new() -> Handler {
        Handler {}
    }
}

impl EventHandler for Handler {
    fn handle(&mut self, event: &Event, _: &mut dyn Engine, graphics: &mut dyn Graphics) {
        if let Event::WindowResize(rectangle) = event {
            graphics.projection().set_viewport(*rectangle);
        }
    }
}
