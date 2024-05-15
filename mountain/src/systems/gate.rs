use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

use crate::model::entrance::Entrance;
use crate::model::gate::Gate;
use crate::model::skiing::{Plan, State};

pub fn run(
    plans: &HashMap<usize, Plan>,
    gates: &HashMap<usize, Gate>,
    entrances: &HashMap<usize, Entrance>,
    open: &HashSet<usize>,
    targets: &mut HashMap<usize, usize>,
    global_targets: &mut HashMap<usize, usize>,
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

        let Some(gate) = gates.get(&target_id) else {
            continue;
        };

        let Some(entrance) = entrances.get(&target_id) else {
            continue;
        };

        if !open.contains(&target_id) {
            continue;
        }

        if gate
            .footprint
            .iter()
            .any(|position| position == *plan_position)
        {
            targets.remove(plan_id);
            if let Entry::Occupied(entry) = global_targets.entry(*plan_id) {
                if *entry.get() == target_id {
                    entry.remove();
                }
            }
            locations.insert(*plan_id, entrance.destination_piste_id);
        }
    }
}
