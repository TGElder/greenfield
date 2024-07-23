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

impl<G> EventHandler<G> for Handler
where
    G: Graphics,
{
    fn handle(&mut self, event: &Event, _: &mut dyn Engine, graphics: &mut G) {
        if let Event::WindowResize(rectangle) = event {
            graphics.projection().set_viewport(*rectangle);
        }
    }
}
