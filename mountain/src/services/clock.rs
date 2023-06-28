use std::time::Instant;

pub struct Service {
    baseline: Instant,
    real_to_game: f32,
}

impl Service {
    pub fn new() -> Service {
        Service {
            baseline: Instant::now(),
            real_to_game: 1.0,
        }
    }

    pub fn get_micros(&self) -> u128 {
        Instant::now()
            .duration_since(self.baseline)
            .mul_f32(self.real_to_game)
            .as_micros()
    }

    pub fn _set_speed(&mut self, real_to_game: f32) {
        self.baseline = Instant::now();
        self.real_to_game = real_to_game;
    }
}
