use crate::{engine::Engine, events::Event, graphics::Graphics};

pub fn handle(event: &Event, _: &mut dyn Engine, graphics: &mut dyn Graphics) {
    if let Event::WindowResize(rectangle) = event {
        graphics.projection().set_viewport(*rectangle);
    }
}
