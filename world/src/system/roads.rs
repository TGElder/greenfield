use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

use crate::model::allocation::Allocation;
use crate::model::path::Path;
use commons::geometry::XY;
use commons::grid::Grid;

pub fn run(
    allocation: &[Allocation],
    paths: &HashMap<(XY<u32>, XY<u32>), Path>,
    routes: &HashMap<(XY<u32>, XY<u32>), Path>,
    towns: &Grid<bool>,
    roads: &mut Grid<bool>,
    links: &mut HashSet<(XY<u32>, XY<u32>)>,
) {
    let mut traffic: HashMap<(XY<u32>, XY<u32>), usize> = HashMap::new();
    let mut new_roads = roads.map(|_, _| false);

    for Allocation {
        from,
        from_market,
        to_market,
        ..
    } in allocation
    {
        *traffic.entry((*from, *from_market)).or_default() += 1;

        for link in routes[&(*from_market, *to_market)].tiles.windows(2) {
            *traffic.entry((link[0], link[1])).or_default() += 1;
        }
    }

    let mut unlinked = traffic
        .iter()
        .filter(|(link, _)| !links.contains(link))
        .collect::<Vec<_>>();

    unlinked.sort_by_key(|(link, traffic)| Reverse((*traffic, paths[link].cost)));

    let mut new_link_count = 0;

    for (link, _) in unlinked {
        let tiles = &paths[link].tiles;
        if link_does_not_touch_any_other_new_link(tiles, towns, &new_roads) {
            links.insert(*link);
            links.insert((link.1, link.0));
            for tile in tiles {
                new_roads[tile] = true;
            }
            new_link_count += 1;
        }
    }

    println!("{} new links", new_link_count);

    *roads = roads.map(|xy, &is_road| is_road || new_roads[xy]);
}

fn link_does_not_touch_any_other_new_link(
    tiles: &[XY<u32>],
    towns: &Grid<bool>,
    new_roads: &Grid<bool>,
) -> bool {
    tiles
        .iter()
        .filter(|&tile| !towns[tile])
        .all(|position| tile_does_not_touch_any_other_new_link(position, towns, new_roads))
}

fn tile_does_not_touch_any_other_new_link(
    tile: &XY<u32>,
    towns: &Grid<bool>,
    new_roads: &Grid<bool>,
) -> bool {
    !towns
        .neighbours_4(tile)
        .any(|neighbour| !towns[neighbour] && new_roads[neighbour])
}
