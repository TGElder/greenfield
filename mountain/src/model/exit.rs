use std::collections::HashSet;

use commons::geometry::XY;

pub struct Exit {
    pub id: usize,
    pub positions: HashSet<XY<u32>>,
}
