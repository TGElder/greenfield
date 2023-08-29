use std::collections::HashMap;

use commons::geometry::{xy, XY};
use commons::origin_grid::OriginGrid;

use crate::model::skiing::State;

const CORNERS: [XY<i32>; 4] = [xy(0, 0), xy(-1, 0), xy(0, -1), xy(-1, -1)];

pub struct Piste {
    pub grid: OriginGrid<bool>,
}

impl Piste {
    pub fn is_on_piste(&self, position: &XY<u32>) -> bool {
        self.grid
            .offsets(position, &CORNERS)
            .any(|corner| self.grid.in_bounds(corner) && self.grid[corner])
    }
}

pub struct PisteCosts {
    lift_to_costs: HashMap<usize, HashMap<State, u64>>,
}

impl PisteCosts {
    pub fn new() -> PisteCosts {
        PisteCosts {
            lift_to_costs: HashMap::new(),
        }
    }

    pub fn costs(&self, lift: &usize) -> Option<&HashMap<State, u64>> {
        self.lift_to_costs.get(lift)
    }

    pub fn set_costs(&mut self, lift: usize, costs: HashMap<State, u64>) {
        self.lift_to_costs.insert(lift, costs);
    }
}
