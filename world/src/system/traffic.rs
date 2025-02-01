use std::collections::HashMap;

use crate::model::allocation::Allocation;
use crate::model::path::Path;
use commons::geometry::XY;
use commons::grid::Grid;

pub fn run(
    allocation: &[Allocation],
    paths: &HashMap<(XY<u32>, XY<u32>), Path>,
    routes: &HashMap<(XY<u32>, XY<u32>), Path>,
    traffic: &mut Grid<usize>,
) {
    *traffic = traffic.map(|_, _| 0);

    for Allocation {
        from,
        from_market,
        to_market,
        ..
    } in allocation
    {
        for tile in paths[&(*from, *from_market)].tiles.iter() {
            traffic[tile] += 1;
        }

        for pair in routes[&(*from_market, *to_market)].tiles.windows(2) {
            for tile in paths[&(pair[0], pair[1])].tiles.iter() {
                traffic[tile] += 1;
            }
        }
    }
}
