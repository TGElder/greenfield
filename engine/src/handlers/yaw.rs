use std::f32::consts::PI;

use crate::{
    engine::Engine,
    events::{ButtonState, Event, EventHandler, KeyboardKey, MouseButton},
    graphics::Graphics,
};

pub struct Handler {
    angle: usize,
    angles: usize,
    key_plus: KeyboardKey,
    key_minus: KeyboardKey,
    mouse_xy: Option<(u32, u32)>,
    selection: Option<[f32; 3]>,
}

pub struct Parameters {
    pub initial_angle: usize,
    pub angles: usize,
    pub key_plus: KeyboardKey,
    pub key_minus: KeyboardKey,
}

impl Handler {
    pub fn new(
        Parameters {
            initial_angle: angle,
            angles,
            key_plus,
            key_minus,
        }: Parameters,
    ) -> Handler {
        Handler {
            angle,
            angles,
            key_plus,
            key_minus,
            mouse_xy: None,
            selection: None,
        }
    }

    fn yaw(&self) -> f32 {
        (self.angle as f32 / self.angles as f32) * PI * 2.0
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
            Event::KeyboardInput {
                key,
                state: ButtonState::Pressed,
            } => {
                let plus = if *key == self.key_plus {
                    true
                } else if *key == self.key_minus {
                    false
                } else {
                    return;
                };

                let Some(mouse_xy) = self.mouse_xy else {return};
                let Ok(xyz) = graphics.world_xyz_at(&mouse_xy) else {return};

                if plus {
                    self.angle = (self.angle + 1) % self.angles;
                } else if *key == self.key_minus {
                    self.angle = (self.angle + self.angles - 1) % self.angles;
                }

                graphics.yaw(self.yaw());
                graphics.look_at(&xyz, &mouse_xy);
            }
            Event::MouseInput {
                button: MouseButton::Left,
                state: ButtonState::Released,
            } => self.selection = None,
            _ => (),
        }
    }
}
