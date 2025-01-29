use std::collections::{HashMap, HashSet};

use commons::geometry::XY;
use commons::grid::Grid;
use network::algorithms::costs_to_targets::{Cost, CostsToTargets};

use crate::model::resource::Resource;
use crate::model::source::Source;
use crate::utils::Network;

pub fn run(
    towns: &Grid<bool>,
    cliff_rise: f32,
    tile_heights: &Grid<f32>,
    resources: &Grid<Option<Resource>>,
    distances: &mut Grid<HashMap<XY<u32>, u64>>,
    markets: &mut Grid<Vec<Source>>,
) {
    let network = Network {
        cliff_rise,
        tile_heights,
    };

    let towns = towns.iter().filter(|xy| towns[xy]).collect::<HashSet<_>>();

    let costs = network.costs_to_targets(&towns, None, None);

    for (
        tile,
        Cost {
            closest_target,
            cost_to_target,
        },
    ) in costs
    {
        distances[tile].insert(closest_target, cost_to_target);

        if let Some(resource) = resources[tile] {
            markets[closest_target].push(Source {
                _tile: tile,
                _resource: resource,
            });
        }
    }
}
