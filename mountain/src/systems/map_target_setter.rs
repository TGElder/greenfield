use std::collections::HashMap;

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::model::skier::Skier;

pub fn run(
    skiers: &HashMap<usize, Skier>,
    targets: &HashMap<usize, usize>,
    map_targets: &mut HashMap<usize, usize>,
) {
    let mut rng = thread_rng();
    for (skier_id, skier) in skiers {
        if map_targets.contains_key(skier_id) {
            continue;
        }

        let candidates = skiers
            .iter()
            .filter(|(_, other_skier)| other_skier.ability == skier.ability)
            .flat_map(|(other_skier_id, _)| targets.get(other_skier_id))
            .copied()
            .collect::<Vec<_>>();

        let choice = candidates.choose(&mut rng);

        if let Some(choice) = choice {
            println!("{} is targetting {}", skier_id, choice);
            map_targets.insert(*skier_id, *choice);
        }
    }
}
