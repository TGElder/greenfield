use std::collections::{HashMap, HashSet};

use commons::grid::Grid;

use crate::model::resource::{Resource, RESOURCES};
use crate::model::source::Source;

pub fn run(
    towns: &Grid<bool>,
    supply: &Grid<Vec<Source>>,
    demand: &Grid<Vec<Source>>,
    prices: &mut Grid<HashMap<Resource, f32>>,
) {
    let towns = towns.iter().filter(|xy| towns[xy]).collect::<HashSet<_>>();

    for town in towns {
        for resource in RESOURCES {
            let demand = demand[town]
                .iter()
                .filter(|source| source.resource == resource)
                .count();
            let supply = supply[town]
                .iter()
                .filter(|source| source.resource == resource)
                .count();
            let price = prices[town][&resource];

            let new_price = price * 1.01f32.powf(demand as f32 - supply as f32);

            *(prices[town].get_mut(&resource).unwrap()) = new_price;
        }
    }
}
