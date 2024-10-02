use std::f32::consts::PI;

use commons::geometry::{XY, XYZ};

use crate::binding::Binding;
use crate::{engine::Engine, events::Event, graphics::Graphics};

pub struct Handler {
    angle: i32,
    angles: i32,
    step_angles: i32,
    mouse_xy: Option<XY<u32>>,
    mouse_yaw_origin: Option<Origin>,
}

pub struct Parameters {
    pub initial_angle: i32,
    pub angles: i32,
    pub step_angles: i32,
}

pub struct Bindings {
    pub step_plus: Binding,
    pub step_minus: Binding,
    pub mouse_yaw_enable: Binding,
    pub mouse_yaw_disable: Binding,
}

pub struct Origin {
    angle: i32,
    mouse_xy: XY<u32>,
    world_xyz: XYZ<f32>,
}

impl Handler {
    pub fn new(
        Parameters {
            initial_angle: angle,
            angles,
            step_angles,
        }: Parameters,
    ) -> Handler {
        Handler {
            angle,
            angles,
            step_angles,
            mouse_xy: None,
            mouse_yaw_origin: None,
        }
    }

    fn step_angle(&mut self, positive: bool, graphics: &mut dyn Graphics) {
        let Some(mouse_xy) = self.mouse_xy else {
            return;
        };
        let Ok(world_xyz) = graphics.world_xyz_at(&mouse_xy) else {
            return;
        };

        if positive {
            self.angle = (self.angle + self.step_angles) % self.angles;
        } else {
            self.angle = (self.angle + self.angles - self.step_angles) % self.angles;
        }

        graphics.projection().yaw(self.compute_yaw());
        graphics.look_at(&world_xyz, &mouse_xy);
    }

    fn compute_yaw(&self) -> f32 {
        (self.angle as f32 / self.angles as f32) * PI * 2.0
    }

    fn mouse_yaw_origin(&mut self, graphics: &mut dyn Graphics) -> Option<Origin> {
        let mouse_xy = self.mouse_xy?;
        Some(Origin {
            angle: self.angle,
            world_xyz: graphics.world_xyz_at(&mouse_xy).ok()?,
            mouse_xy,
        })
    }

    fn mouse_yaw(&mut self, graphics: &mut dyn Graphics) {
        let Some(origin) = &self.mouse_yaw_origin else {
            return;
        };
        let Some(mouse_xy) = self.mouse_xy else {
            return;
        };

        let delta = origin.mouse_xy.x as i32 - mouse_xy.x as i32;
        self.angle = (origin.angle + delta) % self.angles;

        graphics.projection().yaw(self.compute_yaw());
        graphics.look_at(&origin.world_xyz, &mouse_xy);
    }
}
impl Handler {
    pub fn handle(
        &mut self,
        bindings: &Bindings,
        event: &Event,
        _: &mut dyn Engine,
        graphics: &mut dyn Graphics,
    ) {
        if bindings.mouse_yaw_enable.binds_event(event) {
            self.mouse_yaw_origin = self.mouse_yaw_origin(graphics);
        }
        if bindings.mouse_yaw_disable.binds_event(event) {
            self.mouse_yaw_origin = None;
        }

        if let Event::MouseMoved(xy) = event {
            self.mouse_xy = Some(*xy);
            if self.mouse_yaw_origin.is_some() {
                self.mouse_yaw(graphics);
            }
        }

        if self.mouse_yaw_origin.is_none() {
            if bindings.step_plus.binds_event(event) {
                self.step_angle(true, graphics);
            }

            if bindings.step_minus.binds_event(event) {
                self.step_angle(false, graphics);
            }
        }
    }
}
