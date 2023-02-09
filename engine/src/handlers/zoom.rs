use crate::{
    engine::Engine,
    events::{ButtonState, Event, EventHandler, KeyboardKey},
    graphics::Graphics,
};

const LEVELS: [f32; 17] = [
    1.0 / 256.0,
    1.0 / 128.0,
    1.0 / 64.0,
    1.0 / 32.0,
    1.0 / 16.0,
    1.0 / 8.0,
    1.0 / 4.0,
    1.0 / 2.0,
    1.0,
    2.0,
    4.0,
    8.0,
    16.0,
    32.0,
    64.0,
    128.0,
    256.0,
];

pub struct Handler {
    level: usize,
    key_plus: KeyboardKey,
    key_minus: KeyboardKey,
    mouse_xy: Option<(u32, u32)>,
}

pub struct Parameters {
    pub initial_level: usize,
    pub key_plus: KeyboardKey,
    pub key_minus: KeyboardKey,
}

impl Handler {
    pub fn new(
        Parameters {
            initial_level: level,
            key_plus,
            key_minus,
        }: Parameters,
    ) -> Handler {
        Handler {
            level,
            key_plus,
            key_minus,
            mouse_xy: None,
        }
    }

    fn compute_scale(&self) -> f32 {
        LEVELS[self.level]
    }
}
impl EventHandler for Handler {
    fn handle(&mut self, event: &Event, _: &mut dyn Engine, graphics: &mut dyn Graphics) {
        match event {
            Event::MouseMoved(xy) => {
                self.mouse_xy = Some(*xy);
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

                if plus && self.level < LEVELS.len() - 1 {
                    self.level += 1;
                } else if self.level > 0 {
                    self.level -= 1;
                }

                graphics.scale(self.compute_scale());
                graphics.look_at(&xyz, &mouse_xy);
            }
            _ => (),
        }
    }
}
