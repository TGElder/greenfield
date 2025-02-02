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
    sea_level: f32,
    cliff_rise: f32,
    tile_heights: &Grid<f32>,
    roads: &Grid<bool>,
    resources: &Grid<Option<Resource>>,
    markets: &mut Grid<Vec<Source>>,
    paths: &mut HashMap<(XY<u32>, XY<u32>), Path>,
) {
    let network = Network {
        sea_level,
        cliff_rise,
        tile_heights,
        roads,
    };

    let towns = towns.iter().filter(|xy| towns[xy]).collect::<HashSet<_>>();

    let costs = network.costs_to_targets(&towns, None, None);

    for (
        &tile,
        &Cost {
            closest_target,
            cost_to_target,
        },
    ) in costs.iter()
    {
        if let Some(resource) = resources[tile] {
            markets[closest_target].push(Source {
                tile,
                resource,
                cost: cost_to_target,
            });
            paths.insert(
                (tile, closest_target),
                path_from_source(&tile, sea_level, cliff_rise, tile_heights, roads, &costs),
            );
        }
    }
}

fn path_from_source(
    &source: &XY<u32>,
    sea_level: f32,
    cliff_rise: f32,
    tile_heights: &Grid<f32>,
    roads: &Grid<bool>,
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
            .filter(|candidate| {
                cost(
                    &focus,
                    candidate,
                    tile_heights,
                    sea_level,
                    cliff_rise,
                    roads,
                )
                .is_some()
            })
            .min_by_key(|n| costs[n].cost_to_target)
            .unwrap();
    }

    tiles.push(target);

    Path {
        tiles,
        cost: costs[&focus].cost_to_target,
    }
}
