use commons::geometry::{XY, XYZ};

use crate::{
    engine::Engine,
    events::{ButtonState, Event, EventHandler, MouseButton},
    graphics::Graphics,
};

pub struct Handler {
    mouse_xy: Option<XY<u32>>,
    selection: Option<XYZ<f32>>,
}

impl Handler {
    pub fn new() -> Handler {
        Handler {
            mouse_xy: None,
            selection: None,
        }
    }
}

impl Default for Handler {
    fn default() -> Self {
        Self::new()
    }
}

impl EventHandler for Handler {
    fn handle(&mut self, event: &Event, _: &mut dyn Engine, graphics: &mut dyn Graphics) {
        match event {
            Event::MouseMoved(xy) => {
                self.mouse_xy = Some(*xy);
                if let Some(selection) = self.selection {
                    graphics.look_at(&selection, xy);
                }
            }
            Event::MouseInput {
                button: MouseButton::Left,
                state: ButtonState::Pressed,
            } => {
                let Some(mouse_xy) = self.mouse_xy else {return};
                if let Ok(xyz) = graphics.world_xyz_at(&mouse_xy) {
                    self.selection = Some(xyz)
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
