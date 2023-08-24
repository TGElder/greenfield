use std::collections::HashMap;

use crate::model::lift::Lift;
use crate::model::skiing::{Plan, State};

pub fn run(
    plans: &HashMap<usize, Plan>,
    lifts: &HashMap<usize, Lift>,
    targets: &mut HashMap<usize, usize>,
    locations: &mut HashMap<usize, usize>,
) {
    for (id, plan) in plans {
        let Plan::Stationary(State { position, .. }) = plan else {
            continue;
        };
        let Some(target) = targets.get(id) else {
            continue;
        };
        match lifts.get(target) {
            Some(Lift { from, .. }) if from == position => {
                locations.insert(*id, *target);
                targets.remove(id);
            }
            _ => (),
        }
    }
}
