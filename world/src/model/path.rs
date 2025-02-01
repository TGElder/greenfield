use commons::geometry::XY;

pub struct Path {
    pub tiles: Vec<XY<u32>>,
    pub cost: u64,
}
