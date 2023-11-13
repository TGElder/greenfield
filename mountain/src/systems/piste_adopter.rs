use std::collections::HashMap;

use commons::geometry::xy;
use commons::grid::Grid;

use crate::model::skiing::{Plan, State};

pub fn run(
    plans: &HashMap<usize, Plan>,
    piste_map: &Grid<Option<usize>>,
    locations: &mut HashMap<usize, usize>,
) {
    for (id, plan) in plans.iter() {
        if locations.contains_key(id) {
            continue;
        }
        let Plan::Stationary(State { position, .. }) = plan else {
            continue;
        };

        let piste = piste_map
            .offsets(position, &[xy(-1, -1), xy(0, -1), xy(-1, 0), xy(0, 0)])
            .flat_map(|cell| piste_map[cell])
            .next();

        if let Some(piste) = piste {
            locations.insert(*id, piste);
        }
    }
}
