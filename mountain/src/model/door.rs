use std::collections::HashSet;

use commons::geometry::{XYRectangle, XY};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Door {
    pub building_id: usize,
    pub piste_id: usize,
    pub footprint: XYRectangle<u32>,
    pub aperture: HashSet<XY<u32>>,
}
