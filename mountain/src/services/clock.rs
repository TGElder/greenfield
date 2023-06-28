use std::time::Instant;

pub struct Service {
    baseline: Instant,
    real_to_game: f32,
    offset_micros: u128,
}

impl Service {
    pub fn new() -> Service {
        Service {
            baseline: Instant::now(),
            real_to_game: 1.0,
            offset_micros: 0,
        }
    }

    pub fn get_micros(&self) -> u128 {
        self.get_micros_at(&Instant::now())
    }

    fn get_micros_at(&self, instant: &Instant) -> u128 {
        instant
            .duration_since(self.baseline)
            .mul_f32(self.real_to_game)
            .as_micros()
            + self.offset_micros
    }

    pub fn _set_speed(&mut self, real_to_game: f32) {
        let new_baseline = Instant::now();
        self.offset_micros = self.get_micros_at(&new_baseline);
        self.baseline = new_baseline;
        self.real_to_game = real_to_game;
    }
}
