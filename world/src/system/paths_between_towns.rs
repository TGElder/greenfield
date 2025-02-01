use std::collections::{HashMap, HashSet};

use commons::geometry::XY;
use commons::grid::Grid;
use network::algorithms::costs_to_targets::{self, CostsToTargets};

use crate::model::path::Path;
use crate::utils::{cost, Network};

pub fn run(
    towns: &Grid<bool>,
    cliff_rise: f32,
    tile_heights: &Grid<f32>,
    paths: &mut HashMap<(XY<u32>, XY<u32>), Path>,
) {
    let network = Network {
        cliff_rise,
        tile_heights,
    };

    let towns = towns.iter().filter(|xy| towns[xy]).collect::<HashSet<_>>();

    for town in &towns {
        let costs = network
            .costs_to_targets(&HashSet::from([*town]), None, Some(50000))
            .drain()
            .map(
                |(position, costs_to_targets::Cost { cost_to_target, .. })| {
                    (position, cost_to_target)
                },
            )
            .collect::<HashMap<_, _>>();

        for (neighbour, path) in paths_to_town(town, cliff_rise, tile_heights, &towns, &costs) {
            paths.insert((neighbour, *town), path);
        }
    }
}

fn paths_to_town(
    &town: &XY<u32>,
    cliff_rise: f32,
    tile_heights: &Grid<f32>,
    towns: &HashSet<XY<u32>>,
    costs: &HashMap<XY<u32>, u64>,
) -> HashMap<XY<u32>, Path> {
    let neighbours = costs
        .keys()
        .filter(|town| towns.contains(town))
        .copied()
        .collect::<HashSet<_>>();

    let mut out = HashMap::new();

    for neighbour in neighbours {
        let mut tiles = vec![];

        let mut focus = neighbour;
        while focus != town {
            tiles.push(focus);

            focus = tile_heights
                .neighbours_4(focus)
                .filter(|candidate| costs.contains_key(candidate))
                .filter(|candidate| costs[candidate] < costs[&focus])
                .filter(|candidate| cost(&focus, candidate, tile_heights, cliff_rise).is_some())
                .min_by_key(|n| costs[n])
                .unwrap();
        }

        out.insert(
            neighbour,
            Path {
                _tiles: tiles,
                cost: costs[&neighbour],
            },
        );
    }

    out
}
