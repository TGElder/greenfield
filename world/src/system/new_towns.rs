use std::collections::HashSet;

use commons::grid::Grid;
use network::algorithms::costs_to_targets::CostsToTargets;

use crate::utils::Network;

const TRAFFIC_THRESHOLD: usize = 4;
const MIN_DISTANCE: u64 = 25000;
const MAX_DISTANCE: u64 = 50000;

pub fn run(
    sea_level: f32,
    cliff_rise: f32,
    tile_heights: &Grid<f32>,
    roads: &Grid<bool>,
    traffic: &Grid<usize>,
    distances: &Grid<u64>,
    towns: &mut Grid<bool>,
    population: &mut Grid<f32>,
) {
    let mut candidates = traffic
        .iter()
        .filter(|tile| !towns[tile])
        .filter(|tile| traffic[tile] >= TRAFFIC_THRESHOLD)
        .filter(|tile| distances[tile] >= MIN_DISTANCE)
        .filter(|tile| distances[tile] <= MAX_DISTANCE)
        .map(|xy| (xy, (traffic[xy], distances[xy])))
        .collect::<Vec<_>>();

    let mut reserved = towns.map(|_, _| false);
    let network = Network {
        sea_level,
        cliff_rise,
        tile_heights,
        roads,
    };

    candidates.sort_by_key(|&(_, score)| score);

    let mut new_town_count = 0;
    while let Some((tile, _)) = candidates.pop() {
        if !reserved[tile] {
            new_town_count += 1;
            towns[tile] = true;
            population[tile] = 1.0;
            for tile in network
                .costs_to_targets(&HashSet::from([tile]), None, Some(MIN_DISTANCE))
                .keys()
            {
                reserved[tile] = true;
            }
        }
    }

    println!("{} new towns", new_town_count);
}
