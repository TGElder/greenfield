use std::collections::HashSet;

use commons::geometry::XY;

pub struct Target {
    pub id: usize,
    pub positions: HashSet<XY<u32>>,
}
