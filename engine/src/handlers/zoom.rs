use commons::geometry::XY;

use crate::{
    engine::Engine,
    events::{Button, ButtonState, Event, EventHandler},
    graphics::Graphics,
};

pub struct Handler {
    level: i32,
    min_level: i32,
    max_level: i32,
    button_plus: Button,
    button_minus: Button,
    mouse_xy: Option<XY<u32>>,
}

pub struct Parameters {
    pub initial_level: i32,
    pub min_level: i32,
    pub max_level: i32,
    pub button_plus: Button,
    pub button_minus: Button,
}

impl Handler {
    pub fn new(
        Parameters {
            initial_level: level,
            min_level,
            max_level,
            button_plus,
            button_minus,
        }: Parameters,
    ) -> Handler {
        Handler {
            level,
            min_level,
            max_level,
            button_plus,
            button_minus,
            mouse_xy: None,
        }
    }

    fn compute_zoom(&self) -> f32 {
        2.0f32.powi(self.level)
    }
}
impl EventHandler for Handler {
    fn handle(&mut self, event: &Event, _: &mut dyn Engine, graphics: &mut dyn Graphics) {
        match event {
            Event::MouseMoved(xy) => {
                self.mouse_xy = Some(*xy);
            }
            Event::Button {
                button,
                state: ButtonState::Pressed,
            } => {
                let plus = if *button == self.button_plus {
                    true
                } else if *button == self.button_minus {
                    false
                } else {
                    return;
                };
                let Some(mouse_xy) = self.mouse_xy else {return};
                let Ok(xyz) = graphics.world_xyz_at(&mouse_xy) else {return};

                if plus && self.level < self.max_level {
                    self.level += 1;
                } else if !plus && self.level > self.min_level {
                    self.level -= 1;
                }

                graphics.projection().zoom(self.compute_zoom());
                graphics.look_at(&xyz, &mouse_xy);
            }
            _ => (),
        }
    }
}
