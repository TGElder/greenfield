use engine::binding::Binding;

use crate::services::clock;

pub struct Handler {
    pub power: i32,
}

pub struct Bindings {
    pub slow_down: Binding,
    pub speed_up: Binding,
}

impl Handler {
    pub fn new() -> Handler {
        Handler { power: 0 }
    }
    pub fn handle(
        &mut self,
        bindings: &Bindings,
        event: &engine::events::Event,
        clock: &mut clock::Service,
    ) {
        if bindings.slow_down.binds_event(event) {
            self.power -= 1;
            clock.set_speed(self.multiplier());
        }
        if bindings.speed_up.binds_event(event) {
            self.power += 1;
            clock.set_speed(self.multiplier());
        }
    }

    fn multiplier(&self) -> f32 {
        2.0f32.powi(self.power)
    }
}
