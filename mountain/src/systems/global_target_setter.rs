use std::collections::{HashMap, HashSet};

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::model::costs::Costs;
use crate::model::door::Door;
use crate::model::lift::Lift;
use crate::model::skier::Skier;
use crate::model::skiing::{Plan, State};

pub struct Parameters<'a> {
    pub skiers: &'a HashMap<usize, Skier>,
    pub plans: &'a HashMap<usize, Plan>,
    pub locations: &'a HashMap<usize, usize>,
    pub lifts: &'a HashMap<usize, Lift>,
    pub doors: &'a HashMap<usize, Door>,
    pub costs: &'a HashMap<usize, Costs<State>>,
    pub global_costs: &'a Costs<usize>,
    pub global_targets: &'a mut HashMap<usize, usize>,
}

pub fn run(
    Parameters {
        skiers,
        plans,
        locations,
        lifts,
        doors,
        costs,
        global_costs,
        global_targets,
    }: Parameters<'_>,
) {
    let mut rng = thread_rng();

    let lift_pick_ups = lifts
        .values()
        .map(|lift| lift.pick_up.id)
        .collect::<HashSet<_>>();

    for (
        skier_id,
        Skier {
            ability: skier_ability,
            hotel_id,
            ..
        },
    ) in skiers
    {
        if global_targets.contains_key(skier_id) {
            continue;
        }

        let Some(Plan::Stationary(state)) = plans.get(skier_id) else {
            continue;
        };

        let Some(location) = locations.get(skier_id) else {
            continue;
        };

        let Some(costs) = costs.get(location) else {
            continue;
        };
        let stationary_state = state.stationary();

        let door_ids = doors
            .iter()
            .filter(|(_, door)| door.building_id == *hotel_id)
            .map(|(door_id, _)| door_id)
            .collect::<HashSet<_>>();

        let candidates = costs
            .targets_reachable_from_node(&stationary_state, skier_ability)
            .flat_map(|(piste_target, _)| {
                global_costs
                    .targets_reachable_from_node(piste_target, skier_ability)
                    .map(|(target, _)| target)
                    .filter(|target| lift_pick_ups.contains(target))
            })
            // skier must always be able to return home
            .filter(|global_target| {
                global_costs
                    .targets_reachable_from_node(global_target, skier_ability)
                    .any(|(target, _)| door_ids.contains(target))
            })
            .collect::<HashSet<_>>()
            .drain()
            .collect::<Vec<_>>();

        if let Some(&&new_target) = candidates.choose(&mut rng) {
            global_targets.insert(*skier_id, new_target);
        }
    }
}
