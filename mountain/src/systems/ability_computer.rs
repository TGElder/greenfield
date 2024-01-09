use std::collections::{HashMap, HashSet};

use crate::model::direction::Direction;
use crate::model::entrance::Entrance;
use crate::model::lift::Lift;
use crate::model::piste::{Basins, Piste};
use crate::model::skiing::{Ability, State, ABILITIES};

pub fn run(
    pistes: &HashMap<usize, Piste>,
    entrances: &HashMap<usize, Entrance>,
    lifts: &HashMap<usize, Lift>,
    basins: &HashMap<usize, HashMap<Ability, Basins>>,
) {
    for (piste_id, piste) in pistes.iter() {
        println!("Links for piste {}", piste_id);
        let Some(basins) = basins.get(piste_id) else {
            return;
        };
        let lifts_iter = lifts.iter().map(|(_, lift)| lift.drop_off.position);
        let entrances_iter = entrances
            .values()
            .filter(|entrance| entrance.piste == *piste_id)
            .flat_map(|entrance| entrance.footprint.iter());
        let entrances = lifts_iter
            .chain(entrances_iter)
            .filter(|position| piste.grid.in_bounds(position))
            .filter(|position| piste.grid[position])
            .map(|position| State {
                position,
                velocity: 0,
                travel_direction: Direction::North,
            })
            .collect::<HashSet<_>>();

        println!("{:?}", entrances);

        for ability in ABILITIES {
            let links = basins
                .get(&ability)
                .iter()
                .flat_map(|basin| {
                    entrances
                        .iter()
                        .flat_map(|state| basin.targets_reachable_from_state(state))
                })
                .count();
            println!("{:?} links = {}", ability, links);
        }
    }
}
