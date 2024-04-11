use std::collections::{HashMap, HashSet};

use crate::model::ability::Ability;
use crate::model::exit::Exit;
use crate::model::piste::Costs;
use crate::model::skier::Skier;
use crate::model::skiing::{Plan, State};

pub struct Parameters<'a> {
    pub skiers: &'a HashMap<usize, Skier>,
    pub plans: &'a HashMap<usize, Plan>,
    pub locations: &'a HashMap<usize, usize>,
    pub costs: &'a HashMap<usize, Costs<State>>,
    pub open: &'a HashSet<usize>,
    pub exits: &'a HashMap<usize, Vec<Exit>>,
    pub abilities: &'a HashMap<usize, Ability>,
    pub target_costs: &'a Costs<usize>,
    pub map_targets: &'a HashMap<usize, usize>,
    pub targets: &'a mut HashMap<usize, usize>,
}

pub fn run(
    Parameters {
        skiers,
        plans,
        locations,
        costs,
        open,
        exits,
        abilities,
        target_costs,
        map_targets,
        targets,
    }: Parameters<'_>,
) {
    let exits = exits
        .values()
        .flatten()
        .map(|exit| (exit.id, exit))
        .collect::<HashMap<_, _>>();

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

        let Some(map_target) = map_targets.get(skier_id) else {
            continue;
        };

        let Some(target_costs) = target_costs.costs(*map_target, *skier_ability) else {
            continue;
        };

        let candidates = basins
            .targets_reachable_from_state(state, &Ability::Expert)
            .filter(|target| open.contains(target))
            .filter(|target| {
                let Some(exit) = exits.get(target) else {
                    return false;
                };
                let Some(piste_ability) = abilities.get(&exit.destination) else {
                    return false;
                };
                piste_ability <= skier_ability
            })
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
            .min_by_key(|candidate| target_costs.get(candidate).unwrap_or(&u64::MAX));

        if let Some(choice) = choice {
            targets.insert(*skier_id, *choice);
        }
    }
}
