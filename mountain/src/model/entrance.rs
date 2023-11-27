use commons::geometry::XYRectangle;

#[derive(Debug)]
pub struct Entrance {
    pub footprint: XYRectangle<u32>,
    pub piste: usize,
}
