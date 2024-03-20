use std::collections::HashSet;

use commons::geometry::XY;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Exit {
    pub id: usize,
    pub destination: usize,
    pub positions: HashSet<XY<u32>>,
}
