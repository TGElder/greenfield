use std::collections::{HashMap, HashSet};

use crate::model::ability::Ability;
use crate::model::costs::Costs;
use crate::model::skier::Skier;
use crate::model::skiing::{Plan, State};

pub struct Parameters<'a> {
    pub skiers: &'a HashMap<usize, Skier>,
    pub plans: &'a HashMap<usize, Plan>,
    pub locations: &'a HashMap<usize, usize>,
    pub global_costs: &'a Costs<usize>,
    pub costs: &'a HashMap<usize, Costs<State>>,
    pub open: &'a HashSet<usize>,
    pub global_targets: &'a HashMap<usize, usize>,
    pub targets: &'a mut HashMap<usize, usize>,
}

pub fn run(
    Parameters {
        skiers,
        plans,
        locations,
        global_costs,
        costs,
        open,
        global_targets,
        targets,
    }: Parameters<'_>,
) {
    for (skier_id, plan) in plans {
        let Plan::Stationary(state) = plan else {
            continue;
        };

        let Some(Skier {
            ability: skier_ability,
            ..
        }) = skiers.get(skier_id)
        else {
            continue;
        };

        let Some(location_id) = locations.get(skier_id) else {
            continue;
        };

        let Some(basins) = costs.get(location_id) else {
            continue;
        };

        let Some(global_target) = global_targets.get(skier_id) else {
            continue;
        };

        let Some(global_costs) = global_costs.costs(*global_target, *skier_ability) else {
            continue;
        };

        let candidates = basins
            .targets_reachable_from_node(state, &Ability::Expert)
            .map(|(target, _)| target)
            .filter(|target| open.contains(target))
            .collect::<Vec<_>>();

        if let Some(current_target) = targets.get(skier_id) {
            if !candidates.contains(&current_target) {
                println!(
                    "INFO: Removing invalid target {} from {}",
                    current_target, skier_id
                );
                targets.remove(skier_id);
            } else {
                continue;
            }
        }

        let choice = candidates
            .into_iter()
            .flat_map(|candidate| global_costs.get(candidate).map(|cost| (candidate, cost)))
            .min_by_key(|&(_, cost)| *cost)
            .map(|(candidate, _)| candidate);

        if let Some(choice) = choice {
            targets.insert(*skier_id, *choice);
        }
    }
}
