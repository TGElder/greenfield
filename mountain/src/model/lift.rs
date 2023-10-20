use commons::geometry::{XY, XYZ};

pub struct Lift {
    pub nodes: Vec<Node>,
}

pub struct Node {
    pub from: XYZ<f32>,
    pub to: XYZ<f32>,
    pub distance_metres: f32,
    pub from_action: Option<Action>,
}

#[derive(Clone, Copy)]
pub enum Action {
    PickUp(XY<u32>),
    DropOff(XY<u32>),
}
