use std::collections::{HashMap, HashSet};

use commons::geometry::XY;
use commons::grid::Grid;
use network::algorithms::costs_to_targets::CostsToTargets;
use network::model::{Edge, InNetwork};

use crate::utils::cost;

struct Network<'a> {
    cliff_rise: f32,
    tile_heights: &'a Grid<f32>,
}

impl InNetwork<XY<u32>> for Network<'_> {
    fn edges_in<'a>(
        &'a self,
        to: &'a XY<u32>,
    ) -> Box<dyn Iterator<Item = network::model::Edge<XY<u32>>> + 'a> {
        Box::new(self.tile_heights.neighbours_4(to).filter_map(|from| {
            let cost = cost(&from, to, self.tile_heights, self.cliff_rise)?;
            Some(Edge {
                from,
                to: *to,
                cost,
            })
        }))
    }
}

pub fn run(
    towns: &Grid<bool>,
    cliff_rise: f32,
    tile_heights: &Grid<f32>,
) -> Grid<HashMap<XY<u32>, u64>> {
    let mut out = tile_heights.map(|_, _| HashMap::default());

    let network = Network {
        cliff_rise,
        tile_heights,
    };

    let towns = towns.iter().filter(|xy| towns[xy]).collect::<HashSet<_>>();

    for town in &towns {
        let distances = network.costs_to_targets(&HashSet::from([*town]), None, Some(50000));
        for (tile, distance) in distances.iter() {
            out[tile].insert(*town, *distance);
        }
    }

    out
}
