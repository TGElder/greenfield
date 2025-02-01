use std::collections::{HashMap, HashSet};

use crate::model::allocation::Allocation;
use crate::model::path::Path;
use commons::geometry::XY;
use commons::grid::Grid;

pub fn run(
    allocation: &[Allocation],
    paths: &HashMap<(XY<u32>, XY<u32>), Path>,
    routes: &HashMap<(XY<u32>, XY<u32>), Path>,
    roads: &mut Grid<bool>,
    links: &mut HashSet<(XY<u32>, XY<u32>)>,
) {
    let mut traffic: HashMap<(XY<u32>, XY<u32>), usize> = HashMap::new();

    for Allocation {
        from,
        from_market,
        to_market,
        ..
    } in allocation
    {
        *traffic.entry((*from, *from_market)).or_default() += 1;

        for pair in routes[&(*from_market, *to_market)].tiles.windows(2) {
            *traffic.entry((pair[0], pair[1])).or_default() += 1;
        }
    }

    let max = traffic
        .iter()
        .filter(|(pair, _)| !links.contains(pair))
        .max_by_key(|(_, traffic)| *traffic);

    if let Some((pair, _)) = max {
        links.insert(*pair);
        for tile in paths[pair].tiles.iter() {
            roads[tile] = true;
        }
    }
}
