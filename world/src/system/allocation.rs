use std::cmp::Reverse;
use std::collections::HashMap;

use commons::geometry::XY;
use commons::grid::Grid;

use crate::model::allocation::Allocation;
use crate::model::path::Path;
use crate::model::resource::{Resource, RESOURCES};
use crate::model::source::Source;

pub fn run(
    supply: &Grid<Vec<Source>>,
    demand: &Grid<Vec<Source>>,
    routes: &HashMap<(XY<u32>, XY<u32>), Path>,
    allocation: &mut Vec<Allocation>,
) {
    allocation.clear();

    let mut supply = market_to_resource_to_sources(supply);
    let mut demand = market_to_resource_to_sources(demand);

    let mut pairs = routes.iter().collect::<Vec<_>>();
    pairs.sort_by_key(|(_, path)| path.cost);

    for ((from_market, to_market), _) in pairs {
        let Some(supply) = supply.get_mut(from_market) else {
            continue;
        };
        let Some(demand) = demand.get_mut(to_market) else {
            continue;
        };
        for resource in RESOURCES {
            let Some(supply) = supply.get_mut(&resource) else {
                continue;
            };
            let Some(demand) = demand.get_mut(&resource) else {
                continue;
            };

            while !supply.is_empty() && !demand.is_empty() {
                let supply = supply.pop().unwrap();
                demand.pop().unwrap();
                allocation.push(Allocation {
                    from: supply.tile,
                    from_market: *from_market,
                    to_market: *to_market,
                    resource,
                });
            }
        }
    }
}

fn market_to_resource_to_sources(
    sources: &Grid<Vec<Source>>,
) -> HashMap<XY<u32>, HashMap<Resource, Vec<&Source>>> {
    sources
        .iter()
        .filter(|xy| !sources[xy].is_empty())
        .map(|xy| (xy, resource_to_sources(&sources[xy])))
        .collect()
}

fn resource_to_sources(sources: &[Source]) -> HashMap<Resource, Vec<&Source>> {
    let mut out: HashMap<Resource, Vec<&Source>> = HashMap::new();
    for source in sources {
        out.entry(source.resource).or_default().push(source);
    }

    for list in out.values_mut() {
        list.sort_by_key(|source| Reverse(source.cost));
    }

    out
}
