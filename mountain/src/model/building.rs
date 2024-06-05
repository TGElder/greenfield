use commons::geometry::XYRectangle;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Roof {
    Default,
    Rotated,
}

#[derive(Serialize, Deserialize)]
pub struct Building {
    pub footprint: XYRectangle<u32>,
    pub height: u32,
    pub roof: Roof,
    pub under_construction: bool,
}