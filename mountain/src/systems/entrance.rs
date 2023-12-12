use std::collections::{HashMap, HashSet};

use crate::model::entrance::Entrance;
use crate::model::skiing::{Plan, State};

pub fn run(
    plans: &HashMap<usize, Plan>,
    entrances: &HashMap<usize, Entrance>,
    open: &HashSet<usize>,
    targets: &mut HashMap<usize, usize>,
    locations: &mut HashMap<usize, usize>,
) {
    for (plan_id, plan) in plans {
        let Plan::Stationary(State {
            position: plan_position,
            ..
        }) = plan
        else {
            continue;
        };

        let Some(target_id) = targets.get(plan_id).copied() else {
            continue;
        };

        let Some(entrance) = entrances.get(&target_id) else {
            continue;
        };

        if !open.contains(&target_id) {
            continue;
        }

        if entrance
            .footprint
            .iter()
            .any(|position| position == *plan_position)
        {
            targets.remove(plan_id);
            locations.insert(*plan_id, entrance.piste);
        }
    }
}
