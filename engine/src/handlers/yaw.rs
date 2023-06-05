use std::f32::consts::PI;

use commons::geometry::XY;

use crate::events::Button;
use crate::{
    engine::Engine,
    events::{ButtonState, Event, EventHandler},
    graphics::Graphics,
};

pub struct Handler {
    angle: usize,
    angles: usize,
    button_plus: Button,
    button_minus: Button,
    mouse_xy: Option<XY<u32>>,
}

pub struct Parameters {
    pub initial_angle: usize,
    pub angles: usize,
    pub button_plus: Button,
    pub button_minus: Button,
}

impl Handler {
    pub fn new(
        Parameters {
            initial_angle: angle,
            angles,
            button_plus,
            button_minus,
        }: Parameters,
    ) -> Handler {
        Handler {
            angle,
            angles,
            button_plus,
            button_minus,
            mouse_xy: None,
        }
    }

    fn compute_yaw(&self) -> f32 {
        (self.angle as f32 / self.angles as f32) * PI * 2.0
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

                if plus {
                    self.angle = (self.angle + 1) % self.angles;
                } else {
                    self.angle = (self.angle + self.angles - 1) % self.angles;
                }

                graphics.projection().yaw(self.compute_yaw());
                graphics.look_at(&xyz, &mouse_xy);
            }
            _ => (),
        }
    }
}
