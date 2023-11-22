use commons::geometry::XY;

#[derive(Debug)]
pub struct Entrance {
    pub from: XY<u32>,
    pub to: XY<u32>,
    pub piste: usize,
}
