use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Carousel {
    pub lift_id: usize,
    pub velocity: f32,
    pub car_ids: Vec<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Car {
    pub carousel_id: usize,
    pub segment: usize,
    pub distance_from_start_meters: f32,
}
