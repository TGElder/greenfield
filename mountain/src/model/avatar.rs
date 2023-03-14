use commons::geometry::XY;

pub enum Avatar {
    Static(State),
    Moving(Vec<Frame>),
}

pub struct State {
    pub position: XY<u32>,
    pub angle: f32,
}

pub struct Frame {
    pub arrival: u64,
    pub state: State,
}
