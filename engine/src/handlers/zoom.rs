use commons::geometry::XY;

use crate::binding::Binding;
use crate::{
    engine::Engine,
    events::{Event, EventHandler},
    graphics::Graphics,
};

pub struct Handler {
    level: i32,
    min_level: i32,
    max_level: i32,
    pub bindings: Bindings,
    mouse_xy: Option<XY<u32>>,
}

pub struct Parameters {
    pub initial_level: i32,
    pub min_level: i32,
    pub max_level: i32,
    pub bindings: Bindings,
}

pub struct Bindings {
    pub plus: Binding,
    pub minus: Binding,
}

impl Handler {
    pub fn new(
        Parameters {
            initial_level: level,
            min_level,
            max_level,
            bindings,
        }: Parameters,
    ) -> Handler {
        Handler {
            level,
            min_level,
            max_level,
            bindings,
            mouse_xy: None,
        }
    }

    fn step_level(&mut self, positive: bool, graphics: &mut dyn Graphics) {
        let Some(mouse_xy) = self.mouse_xy else {
            return;
        };

        let Ok(xyz) = graphics.world_xyz_at(&mouse_xy) else {
            return;
        };

        if positive && self.level < self.max_level {
            self.level += 1;
        } else if !positive && self.level > self.min_level {
            self.level -= 1;
        }

        graphics.projection().zoom(self.compute_zoom());
        graphics.look_at(&xyz, &mouse_xy);
    }

    fn compute_zoom(&self) -> f32 {
        2.0f32.powi(self.level)
    }
}
impl<G> EventHandler<G> for Handler
where
    G: Graphics,
{
    fn handle(&mut self, event: &Event, _: &mut dyn Engine, graphics: &mut G) {
        if let Event::MouseMoved(xy) = event {
            self.mouse_xy = Some(*xy);
        }

        if self.bindings.plus.binds_event(event) {
            self.step_level(true, graphics);
        }

        if self.bindings.minus.binds_event(event) {
            self.step_level(false, graphics);
        }
    }
}
