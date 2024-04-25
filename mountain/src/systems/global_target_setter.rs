use std::collections::HashMap;

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::model::costs::Costs;
use crate::model::lift::Lift;
use crate::model::skier::Skier;
use crate::model::skiing::{Plan, State};

pub struct Parameters<'a> {
    pub skiers: &'a HashMap<usize, Skier>,
    pub plans: &'a HashMap<usize, Plan>,
    pub locations: &'a HashMap<usize, usize>,
    pub costs: &'a HashMap<usize, Costs<State>>,
    pub lifts: &'a HashMap<usize, Lift>,
    pub global_costs: &'a Costs<usize>,
    pub global_targets: &'a mut HashMap<usize, usize>,
}

pub fn run(
    Parameters {
        skiers,
        plans,
        locations,
        costs,
        lifts,
        global_costs,
        global_targets,
    }: Parameters<'_>,
) {
    let mut rng = thread_rng();

    for (
        skier_id,
        Skier {
            ability: skier_ability,
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

        let candidates = costs
            .targets_reachable_from_node(&stationary_state, skier_ability)
            .flat_map(|(piste_target, _)| {
                global_costs
                    .targets_reachable_from_node(piste_target, skier_ability)
                    .filter(|(target, _)| lifts.contains_key(target)) // only target lifts
                    .filter(|&(_, cost)| *cost != 0) // global target must require moving
                    .filter(move |(new_target, _)| {
                        // will not get stuck
                        global_costs
                            .targets_reachable_from_node(new_target, skier_ability)
                            .filter(|&(_, cost)| *cost != 0)
                            .count()
                            != 0
                    })
            })
            .map(|(global_target, _)| global_target)
            .collect::<Vec<_>>();

        if let Some(&&new_target) = candidates.choose(&mut rng) {
            global_targets.insert(*skier_id, new_target);
        }
    }
}
