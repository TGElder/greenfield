use std::collections::{HashMap, HashSet};

use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

use crate::model::costs::Costs;
use crate::model::door::Door;
use crate::model::lift::Lift;
use crate::model::skier::Skier;
use crate::model::skiing::{Plan, State};

const EXPLORE_RATIO: f32 = 0.75;

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

    let all_door_ids = doors.keys().collect::<HashSet<_>>();

    let lift_drop_offs = lifts
        .values()
        .map(|lift| lift.drop_off.id)
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

        let targets_on_this_piste = costs
            .targets_reachable_from_node(&stationary_state, skier_ability)
            .map(|(piste_target, _)| piste_target);

        let explore = rng.gen::<f32>() <= EXPLORE_RATIO;

        let candidates: HashSet<usize> = if explore {
            targets_on_this_piste
                .filter(|target| !doors.contains_key(target))
                .copied()
                .collect()
        } else {
            targets_on_this_piste
                .flat_map(|piste_target| {
                    global_costs
                        .targets_reachable_from_node(piste_target, skier_ability)
                        .map(|(target, _)| target)
                        .filter(|target| lift_drop_offs.contains(target))
                })
                .copied()
                .collect()
        };

        // Trying to find "safe" candidate from which skier can return to own hotel

        let hotel_door_ids = doors
            .iter()
            .filter(|(_, door)| door.building_id == *hotel_id)
            .map(|(door_id, _)| door_id)
            .collect::<HashSet<_>>();

        let safe_candidates: Vec<&usize> = candidates
            .iter()
            .filter(|global_target| {
                global_costs
                    .targets_reachable_from_node(global_target, skier_ability)
                    .any(|(target, _)| hotel_door_ids.contains(target))
            })
            .collect();

        if let Some(&new_target) = safe_candidates.choose(&mut rng) {
            global_targets.insert(*skier_id, *new_target);
        }

        // Alternatively finding candidate from which skier can return to any hotel

        let safe_candidates: Vec<usize> = candidates
            .into_iter()
            .filter(|global_target| {
                global_costs
                    .targets_reachable_from_node(global_target, skier_ability)
                    .any(|(target, _)| all_door_ids.contains(target))
            })
            .collect();

        if let Some(&new_target) = safe_candidates.choose(&mut rng) {
            global_targets.insert(*skier_id, new_target);
        }
    }
}
