use std::collections::{HashMap, HashSet};

use rand::seq::SliceRandom;

use crate::model::piste::Costs;
use crate::model::skiing::Plan;

pub fn run(
    plans: &HashMap<usize, Plan>,
    locations: &HashMap<usize, usize>,
    basins: &HashMap<usize, Costs>,
    open: &HashSet<usize>,
    targets: &mut HashMap<usize, usize>,
) {
    for (plan_id, plan) in plans {
        let Plan::Stationary(state) = plan else {
            continue;
        };

        let Some(location_id) = locations.get(plan_id) else {
            continue;
        };

        let Some(basins) = basins.get(location_id) else {
            continue;
        };

        let candidates = basins
            .targets_reachable_from_state(state)
            .filter(|target| open.contains(target))
            .collect::<Vec<_>>();

        if let Some(current_target) = targets.get(plan_id) {
            if !candidates.contains(&current_target) {
                println!(
                    "INFO: Removing invalid target {} from {}",
                    current_target, plan_id
                );
                targets.remove(plan_id);
            } else {
                continue;
            }
        }

        let choice = candidates.choose(&mut rand::thread_rng());

        if let Some(choice) = choice {
            targets.insert(*plan_id, **choice);
        }
    }
}
