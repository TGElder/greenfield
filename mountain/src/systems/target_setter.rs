use std::collections::{HashMap, HashSet};

use rand::seq::SliceRandom;

use crate::model::piste::PisteCosts;
use crate::model::skiing::{Mode, Plan, State};

pub fn run(
    plans: &HashMap<usize, Plan>,
    locations: &HashMap<usize, usize>,
    skiing_costs: &HashMap<usize, PisteCosts>,
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

        let Some(costs) = skiing_costs.get(location_id) else {
            continue;
        };

        let state = State {
            mode: Mode::Skiing { velocity: 0 },
            ..*state
        };

        let candidates = costs
            .target_to_costs
            .iter()
            .filter(|(target, _)| open.contains(target))
            .filter(|(_, costs)| costs.contains_key(&state))
            .map(|(target, _)| *target)
            .collect::<Vec<_>>();

        if let Some(target) = targets.get(plan_id) {
            if !candidates.contains(target) {
                println!("Removing target");
                targets.remove(plan_id);
            } else {
                continue;
            }
        }

        let choice = candidates.choose(&mut rand::thread_rng());

        if let Some(choice) = choice {
            println!("Choice is {}", choice);
            targets.insert(*plan_id, *choice);
        }
    }
}
