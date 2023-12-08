use std::time::Instant;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Service {
    #[serde(skip, default = "Instant::now")]
    baseline: Instant,
    speed: f32,
    offset_micros: u128,
}

impl Service {
    pub fn new() -> Service {
        Service {
            baseline: Instant::now(),
            speed: 1.0,
            offset_micros: 0,
        }
    }

    pub fn get_micros(&self) -> u128 {
        self.get_micros_at(&Instant::now())
    }

    fn get_micros_at(&self, instant: &Instant) -> u128 {
        instant
            .duration_since(self.baseline)
            .mul_f32(self.speed)
            .as_micros()
            + self.offset_micros
    }

    pub fn speed(&self) -> f32 {
        self.speed
    }

    pub fn set_speed(&mut self, speed: f32) {
        let new_baseline = Instant::now();
        self.offset_micros = self.get_micros_at(&new_baseline);
        self.baseline = new_baseline;
        self.speed = speed;
    }
}
