use std::collections::{HashMap, HashSet};

use commons::grid::Grid;
use commons::unsafe_ordering::unsafe_ordering;

use crate::model::resource::Resource;
use crate::model::source::Source;

pub fn run(
    towns: &Grid<bool>,
    population: &Grid<f32>,
    markets: &Grid<Vec<Source>>,
    prices: &Grid<HashMap<Resource, f32>>,
    supply: &mut Grid<Vec<Source>>,
) {
    let towns = towns.iter().filter(|xy| towns[xy]).collect::<HashSet<_>>();

    for town in towns {
        let prices = &prices[town];
        let mut opportunities = markets[town].clone();
        opportunities.sort_by(|a, b| unsafe_ordering(&prices[&a.resource], &prices[&b.resource]));
        for _ in 0..population[town].floor() as usize {
            if let Some(opportunity) = opportunities.pop() {
                supply[town].push(opportunity);
            }
        }
    }
}
