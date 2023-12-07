use commons::geometry::XYRectangle;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Entrance {
    pub footprint: XYRectangle<u32>,
    pub piste: usize,
}
