use commons::geometry::{XY, XYZ};

pub struct Lift {
    pub segments: Vec<Segment>,
    pub pick_up: Portal,
    pub drop_off: Portal,
}

pub struct Segment {
    pub from: XYZ<f32>,
    pub to: XYZ<f32>,
    pub length_meters: f32,
}

pub struct Portal {
    pub segment: usize,
    pub position: XY<u32>,
}
