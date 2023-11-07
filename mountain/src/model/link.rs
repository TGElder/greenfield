use std::collections::HashMap;

use commons::geometry::XY;
use commons::origin_grid::OriginGrid;
use network::model::Edge;

use crate::model::skiing::State;

pub struct Link {
    pub grid: OriginGrid<bool>,
    pub edges: HashMap<XY<u32>, Edge<State>>,
    pub from: usize,
    pub to: usize,
}
