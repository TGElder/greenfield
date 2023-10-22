pub struct Carousel {
    pub velocity: f32,
    pub cars: Vec<usize>,
}

#[derive(Debug)]
pub struct Car {
    pub lift_id: usize,
    pub segment: usize,
    pub meters_from_start: f32,
}
