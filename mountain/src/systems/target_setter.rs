use std::collections::{HashMap, HashSet};

use rand::seq::SliceRandom;

use crate::model::ability::Ability;
use crate::model::costs::Costs;
use crate::model::exit::Exit;
use crate::model::skier::Skier;
use crate::model::skiing::Plan;

pub struct Parameters<'a> {
    pub skiers: &'a HashMap<usize, Skier>,
    pub plans: &'a HashMap<usize, Plan>,
    pub locations: &'a HashMap<usize, usize>,
    pub costs: &'a HashMap<usize, Costs>,
    pub open: &'a HashSet<usize>,
    pub exits: &'a HashMap<usize, Vec<Exit>>,
    pub abilities: &'a HashMap<usize, Ability>,
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

        let candidates = basins
            .targets_reachable_from_state(state, &Ability::Expert)
            .map(|(target, _)| target)
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

        let choice = candidates.choose(&mut rand::thread_rng());

        if let Some(choice) = choice {
            targets.insert(*skier_id, **choice);
        }
    }
}
