pub struct Carousel {
    pub velocity: f32,
    pub cars: Vec<usize>,
}

#[derive(Debug)]
pub struct Car {
    pub lift_id: usize,
    pub segment: usize,
    pub distance_from_start_meters: f32,
}
