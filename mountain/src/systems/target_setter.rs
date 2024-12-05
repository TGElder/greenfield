use std::collections::{HashMap, HashSet};

use commons::map::ContainsKeyValue;

use crate::model::costs::Costs;
use crate::model::door::Door;
use crate::model::open;
use crate::model::skier::Skier;
use crate::model::skiing::{Plan, State};
use crate::network::global::GLOBAL_COST_DIVISOR;

pub struct Parameters<'a> {
    pub skiers: &'a HashMap<usize, Skier>,
    pub plans: &'a HashMap<usize, Plan>,
    pub locations: &'a HashMap<usize, usize>,
    pub doors: &'a HashMap<usize, Door>,
    pub global_costs: &'a Costs<usize>,
    pub costs: &'a HashMap<usize, Costs<State>>,
    pub open: &'a HashMap<usize, open::Status>,
    pub global_targets: &'a mut HashMap<usize, usize>,
    pub targets: &'a mut HashMap<usize, usize>,
}

pub fn run(
    Parameters {
        skiers,
        plans,
        locations,
        doors,
        global_costs,
        costs,
        open,
        global_targets,
        targets,
    }: Parameters<'_>,
) {
    let default_global_costs = HashMap::default();

    for (skier_id, plan) in plans {
        let Plan::Stationary(state) = plan else {
            continue;
        };

        let Some(Skier {
            ability: skier_ability,
            hotel_id,
            ..
        }) = skiers.get(skier_id)
        else {
            continue;
        };

        let Some(location_id) = locations.get(skier_id) else {
            continue;
        };

        let Some(costs) = costs.get(location_id) else {
            continue;
        };

        let Some(global_target) = global_targets.get(skier_id) else {
            continue;
        };

        // check skier can return home from global target

        let door_ids = doors
            .iter()
            .filter(|(_, door)| door.building_id == *hotel_id)
            .map(|(door_id, _)| door_id)
            .collect::<HashSet<_>>();

        if !global_costs
            .targets_reachable_from_node(global_target, skier_ability)
            .any(|(target, _)| door_ids.contains(target))
        {
            println!(
                "Skier {} cannot get home from global target {}",
                skier_id, global_target
            );
            targets.remove(skier_id);
            global_targets.remove(skier_id);
            return;
        }

        // find best local target for global target

        let costs_to_global_target = global_costs
            .costs(*global_target, *skier_ability)
            .unwrap_or(&default_global_costs);

        let stationary_state = state.stationary();

        let target = costs
            .targets_reachable_from_node(&stationary_state, skier_ability)
            .filter(|(&target, _)| !open.contains_key_value(target, open::Status::Closed))
            .flat_map(|(target, cost)| {
                costs_to_global_target
                    .get(target)
                    .map(|global_cost| (target, cost + (global_cost * GLOBAL_COST_DIVISOR)))
            })
            .min_by_key(|&(_, cost)| cost)
            .map(|(candidate, _)| candidate);

        if let Some(target) = target {
            targets.insert(*skier_id, *target);
        } else {
            targets.remove(skier_id);
            global_targets.remove(skier_id);
        }
    }
}
