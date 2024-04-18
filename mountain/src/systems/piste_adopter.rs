use std::collections::HashMap;

use commons::grid::{Grid, CORNERS_INVERSE};
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::model::ability::Ability;
use crate::model::skier::Skier;
use crate::model::skiing::{Plan, State};

pub fn run(
    plans: &HashMap<usize, Plan>,
    piste_map: &Grid<Option<usize>>,
    skiers: &HashMap<usize, Skier>,
    abilities: &HashMap<usize, Ability>,
    locations: &mut HashMap<usize, usize>,
) {
    let mut rng = thread_rng();
    for (id, plan) in plans.iter() {
        let Plan::Stationary(State { position, .. }) = plan else {
            continue;
        };

        if let Some(location) = locations.get(id) {
            if piste_map
                .offsets(position, &CORNERS_INVERSE)
                .any(|cell| piste_map[cell] == Some(*location))
            {
                continue;
            }
        }

        let Some(Skier {
            ability: skier_ability,
            ..
        }) = skiers.get(id)
        else {
            continue;
        };

        let candidates = piste_map
            .offsets(position, &CORNERS_INVERSE)
            .flat_map(|cell| piste_map[cell])
            .filter(|piste| {
                abilities
                    .get(piste)
                    .map(|piste_ability| piste_ability <= skier_ability)
                    .unwrap_or_default()
            })
            .collect::<Vec<_>>();

        let piste = candidates.choose(&mut rng);

        if let Some(&piste) = piste {
            locations.insert(*id, piste);
        }
    }
}
