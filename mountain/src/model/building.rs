use commons::geometry::{XYRectangle, XYZ};
use serde::{Deserialize, Serialize};

use crate::model::direction::Direction;

#[derive(Serialize, Deserialize)]
pub struct Building {
    pub footprint: XYRectangle<u32>,
    pub height: u32,
    pub roof: Roof,
    pub under_construction: bool,
    pub windows: Vec<Window>,
}

#[derive(Serialize, Deserialize)]
pub enum Roof {
    Peaked,
    PeakedRotated,
    Flat,
}

#[derive(Serialize, Deserialize)]
pub struct Window {
    pub position: XYZ<f32>,
    pub direction: Direction,
}
