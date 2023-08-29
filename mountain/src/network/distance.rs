use commons::geometry::XY;
use commons::grid::Grid;
use network::model::{Edge, InNetwork};

use crate::model::piste::Piste;

pub struct DistanceNetwork<'a> {
    pub terrain: &'a Grid<f32>,
    pub piste: &'a Piste,
    pub can_visit: &'a dyn Fn(&XY<u32>) -> bool,
}

impl<'a> InNetwork<XY<u32>> for DistanceNetwork<'a> {
    fn edges_in<'b>(
        &'b self,
        to: &'b XY<u32>,
    ) -> Box<dyn Iterator<Item = network::model::Edge<XY<u32>>> + 'b> {
        let iter = self
            .terrain
            .neighbours_8(to)
            .filter(|from| self.piste.is_on_piste(from))
            .filter(|from| (self.can_visit)(from))
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
