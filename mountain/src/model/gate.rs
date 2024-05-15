use commons::geometry::XYRectangle;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Gate {
    pub footprint: XYRectangle<u32>,
}
