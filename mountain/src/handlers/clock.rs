use engine::binding::Binding;

use crate::services::clock;

pub struct Handler {
    pub bindings: Bindings,
    pub power: i32,
}

pub struct Bindings {
    pub slow_down: Binding,
    pub speed_up: Binding,
}

impl Handler {
    pub fn new(bindings: Bindings) -> Handler {
        Handler { bindings, power: 0 }
    }
    pub fn handle(&mut self, event: &engine::events::Event, clock: &mut clock::Service) {
        if self.bindings.slow_down.binds_event(event) {
            self.power -= 1;
            clock.set_real_to_game(self.multiplier());
        }
        if self.bindings.speed_up.binds_event(event) {
            self.power += 1;
            clock.set_real_to_game(self.multiplier());
        }
    }

    fn multiplier(&self) -> f32 {
        2.0f32.powi(self.power)
    }
}
