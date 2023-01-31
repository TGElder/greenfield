use crate::{
    engine::Engine,
    events::{ButtonState, Event, EventHandler, MouseButton},
    graphics::Graphics,
};

pub struct DragHandler {
    mouse_xy: Option<(u32, u32)>,
    selection: Option<u32>,
}

impl DragHandler {
    pub fn new() -> DragHandler {
        DragHandler {
            mouse_xy: None,
            selection: None,
        }
    }
}

impl Default for DragHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl EventHandler for DragHandler {
    fn handle(&mut self, event: &Event, _: &mut dyn Engine, graphics: &mut dyn Graphics) {
        match event {
            Event::MouseMoved(xy) => {
                self.mouse_xy = Some(*xy);
                if let Some(selection) = self.selection {
                    graphics.look_at(selection, xy).unwrap();
                }
            }
            Event::MouseInput {
                button: MouseButton::Left,
                state: ButtonState::Pressed,
            } => {
                let Some(mouse_xy) = self.mouse_xy else {return};
                if let Ok(xy) = graphics.id_at(mouse_xy) {
                    self.selection = Some(xy)
                }
            }
            Event::MouseInput {
                button: MouseButton::Left,
                state: ButtonState::Released,
            } => self.selection = None,
            _ => (),
        }
    }
}
