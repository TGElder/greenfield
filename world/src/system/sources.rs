use std::collections::{HashMap, HashSet};

use commons::geometry::XY;
use commons::grid::Grid;
use network::algorithms::costs_to_targets::{self, Cost, CostsToTargets};

use crate::model::path::Path;
use crate::model::resource::Resource;
use crate::model::source::Source;
use crate::utils::{cost, Network};

pub fn run(
    towns: &Grid<bool>,
    cliff_rise: f32,
    tile_heights: &Grid<f32>,
    resources: &Grid<Option<Resource>>,
    markets: &mut Grid<Vec<Source>>,
    paths: &mut HashMap<(XY<u32>, XY<u32>), Path>,
) {
    let network = Network {
        cliff_rise,
        tile_heights,
    };

    let towns = towns.iter().filter(|xy| towns[xy]).collect::<HashSet<_>>();

    let costs = network.costs_to_targets(&towns, None, None);

    for (&tile, &Cost { closest_target, .. }) in costs.iter() {
        if let Some(resource) = resources[tile] {
            markets[closest_target].push(Source {
                _tile: tile,
                _resource: resource,
            });
            paths.insert(
                (tile, closest_target),
                path_from_source(&tile, cliff_rise, tile_heights, &costs),
            );
        }
    }
}

fn path_from_source(
    &source: &XY<u32>,
    cliff_rise: f32,
    tile_heights: &Grid<f32>,
    costs: &HashMap<XY<u32>, costs_to_targets::Cost<XY<u32>>>,
) -> Path {
    let mut tiles = vec![];

    let mut focus = source;
    let target = costs[&focus].closest_target;
    while focus != target {
        tiles.push(focus);

        focus = tile_heights
            .neighbours_4(focus)
            .filter(|candidate| costs.contains_key(candidate))
            .filter(|candidate| costs[candidate].closest_target == target)
            .filter(|candidate| costs[candidate].cost_to_target < costs[&focus].cost_to_target)
            .filter(|candidate| cost(&focus, candidate, tile_heights, cliff_rise).is_some())
            .min_by_key(|n| costs[n].cost_to_target)
            .unwrap();
    }

    Path {
        _tiles: tiles,
        _cost: costs[&focus].cost_to_target,
    }
}
