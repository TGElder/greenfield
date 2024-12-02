use std::iter::empty;

use commons::geometry::XY;
use commons::grid::OFFSETS_8;
use network::model::{Edge, InNetwork};

use crate::model::piste::Piste;

pub struct DistanceNetwork<'a> {
    pub piste: &'a Piste,
    pub can_visit: &'a dyn Fn(&XY<u32>) -> bool,
}

impl InNetwork<XY<u32>> for DistanceNetwork<'_> {
    fn edges_in<'b>(
        &'b self,
        to: &'b XY<u32>,
    ) -> Box<dyn Iterator<Item = network::model::Edge<XY<u32>>> + 'b> {
        if !(self.can_visit)(to) {
            return Box::new(empty());
        }

        let iter = OFFSETS_8
            .iter()
            .flat_map(move |offset| self.piste.grid.offset(to, offset))
            .filter(|from| self.piste.grid.in_bounds(from))
            .filter(|from| self.piste.grid[from])
            .map(move |from| Edge {
                from,
                to: *to,
                cost: self.cost(&from, to),
            });

        Box::new(iter)
    }
}

impl DistanceNetwork<'_> {
    fn cost(&self, from: &XY<u32>, to: &XY<u32>) -> u32 {
        if from.x == to.x || from.y == to.y {
            1000
        } else {
            1414
        }
    }
}
