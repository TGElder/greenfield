use std::collections::{HashMap, HashSet};

use crate::model::ability::Ability;
use crate::model::direction::Direction;
use crate::model::entrance::Entrance;
use crate::model::exit::Exit;
use crate::model::lift::Lift;
use crate::model::piste::{Costs, Piste};
use crate::model::skiing::State;

pub fn compute_piste(
    piste_id: &usize,
    pistes: &HashMap<usize, Piste>,
    costs: &HashMap<usize, Costs>,
    exits: &HashMap<usize, Vec<Exit>>,
    lifts: &HashMap<usize, Lift>,
    entrances: &HashMap<usize, Entrance>,
    abilities: &mut HashMap<usize, Option<Ability>>,
) {
    abilities.insert(
        *piste_id,
        compute_ability(piste_id, pistes, costs, exits, lifts, entrances),
    );
}

fn compute_ability(
    piste_id: &usize,
    pistes: &HashMap<usize, Piste>,
    costs: &HashMap<usize, Costs>,
    exits: &HashMap<usize, Vec<Exit>>,
    lifts: &HashMap<usize, Lift>,
    entrances: &HashMap<usize, Entrance>,
) -> Option<Ability> {
    let Some(piste) = pistes.get(piste_id) else {
        return None;
    };

    let Some(costs) = costs.get(piste_id) else {
        return None;
    };

    let Some(exits) = exits.get(piste_id) else {
        return None;
    };

    let grid = &piste.grid;

    let lifts_iter = lifts.values().map(|lift| lift.drop_off.position);

    let entrances_iter = entrances
        .values()
        .filter(|entrance| entrance.piste == *piste_id)
        .flat_map(|entrance| entrance.footprint.iter());

    let entrances = lifts_iter
        .chain(entrances_iter)
        .filter(|position| grid.in_bounds(position) && grid[position])
        .map(|position| State {
            position,
            velocity: 0,
            travel_direction: Direction::North,
        })
        .collect::<HashSet<_>>();

    let targets = exits.iter().map(|exit| exit.id).collect::<HashSet<_>>();

    let mut abilities = Vec::with_capacity(entrances.len() * targets.len());
    for entrance in entrances.iter() {
        for target in targets.iter() {
            if let Some(ability) = costs.min_ability(entrance, target) {
                abilities.push(ability);
            } else {
                return None;
            }
        }
    }

    abilities.iter().max().copied()
}
