use std::collections::{HashMap, HashSet};

use commons::geometry::XY;
use commons::grid::Grid;
use network::algorithms::costs_to_targets::CostsToTargets;

use crate::utils::Network;

pub fn run(
    towns: &Grid<bool>,
    cliff_rise: f32,
    tile_heights: &Grid<f32>,
    distances: &mut Grid<HashMap<XY<u32>, u64>>,
) {
    let network = Network {
        cliff_rise,
        tile_heights,
    };

    let towns = towns.iter().filter(|xy| towns[xy]).collect::<HashSet<_>>();

    for town in &towns {
        let town_distances = network.costs_to_targets(&HashSet::from([*town]), None, Some(50000));
        for (tile, cost) in town_distances.iter() {
            distances[tile].insert(*town, cost.cost_to_target);
        }
    }
}
