use commons::geometry::XY;
use commons::grid::Grid;
use commons::grid::OFFSETS_8;
use network::model::{Edge, InNetwork};

use crate::model::piste::Piste;

pub struct DistanceNetwork<'a> {
    terrain: &'a Grid<f32>,
    piste: &'a Piste,
    lifts: &'a Grid<bool>,
}

impl<'a> DistanceNetwork<'a> {
    pub fn new(
        terrain: &'a Grid<f32>,
        piste: &'a Piste,
        lifts: &'a Grid<bool>,
    ) -> DistanceNetwork<'a> {
        DistanceNetwork {
            terrain,
            piste,
            lifts,
        }
    }
}

impl<'a> InNetwork<XY<u32>> for DistanceNetwork<'a> {
    fn edges_in<'b>(
        &'b self,
        to: &'b XY<u32>,
    ) -> Box<dyn Iterator<Item = network::model::Edge<XY<u32>>> + 'b> {
        let iter = OFFSETS_8
            .iter()
            .flat_map(move |offset| self.piste.grid.offset(to, offset))
            .filter(|from| self.piste.grid.in_bounds(from))
            .filter(|from| self.piste.grid[from])
            .filter(|from| !self.lifts[from])
            .map(move |from| Edge {
                from,
                to: *to,
                cost: self.cost(&from, to),
            });

        Box::new(iter)
    }
}

impl<'a> DistanceNetwork<'a> {
    fn cost(&self, from: &XY<u32>, to: &XY<u32>) -> u32 {
        const COST_PER_Z_UNIT: f32 = 1000.0;

        ((self.terrain[from] - self.terrain[to]).abs() * COST_PER_Z_UNIT) as u32
    }
}
