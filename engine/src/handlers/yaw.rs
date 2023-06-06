use std::f32::consts::PI;

use commons::geometry::XY;

use crate::binding::Binding;
use crate::{
    engine::Engine,
    events::{Event, EventHandler},
    graphics::Graphics,
};

pub struct Handler {
    angle: usize,
    angles: usize,
    bindings: Bindings,
    mouse_xy: Option<XY<u32>>,
}

pub struct Parameters {
    pub initial_angle: usize,
    pub angles: usize,
    pub bindings: Bindings,
}

pub struct Bindings {
    pub plus: Binding,
    pub minus: Binding,
}

impl Handler {
    pub fn new(
        Parameters {
            initial_angle: angle,
            angles,
            bindings,
        }: Parameters,
    ) -> Handler {
        Handler {
            angle,
            angles,
            bindings,
            mouse_xy: None,
        }
    }

    fn step_angle(&mut self, positive: bool, graphics: &mut dyn Graphics) {
        let Some(mouse_xy) = self.mouse_xy else {return};
        let Ok(xyz) = graphics.world_xyz_at(&mouse_xy) else {return};

        if positive {
            self.angle = (self.angle + 1) % self.angles;
        } else {
            self.angle = (self.angle + self.angles - 1) % self.angles;
        }

        graphics.projection().yaw(self.compute_yaw());
        graphics.look_at(&xyz, &mouse_xy);
    }

    fn compute_yaw(&self) -> f32 {
        (self.angle as f32 / self.angles as f32) * PI * 2.0
    }
}
impl EventHandler for Handler {
    fn handle(&mut self, event: &Event, _: &mut dyn Engine, graphics: &mut dyn Graphics) {
        if let Event::MouseMoved(xy) = event {
            self.mouse_xy = Some(*xy);
        }

        if self.bindings.plus.binds_event(event) {
            self.step_angle(true, graphics);
        }

        if self.bindings.minus.binds_event(event) {
            self.step_angle(false, graphics);
        }
    }
}
