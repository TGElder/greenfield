use std::collections::HashMap;

use commons::grid::{Grid, CORNERS_INVERSE};

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
            .offsets(position, &CORNERS_INVERSE)
            .flat_map(|cell| piste_map[cell])
            .next();

        if let Some(piste) = piste {
            locations.insert(*id, piste);
        }
    }
}
