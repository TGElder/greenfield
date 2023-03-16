use commons::geometry::XY;

pub enum Avatar {
    Static(State),
}

pub struct State {
    pub position: XY<u32>,
    pub angle: f32,
}
