use std::collections::{HashMap, HashSet};

use crate::model::ability::Ability;
use crate::model::costs::Costs;
use crate::model::direction::Direction;
use crate::model::exit::Exit;
use crate::model::gate::Gate;
use crate::model::lift::Lift;
use crate::model::piste::Piste;
use crate::model::skiing::State;

pub fn compute_piste(
    piste_id: &usize,
    pistes: &HashMap<usize, Piste>,
    costs: &HashMap<usize, Costs<State>>,
    exits: &HashMap<usize, Vec<Exit>>,
    lifts: &HashMap<usize, Lift>,
    gates: &HashMap<usize, Gate>,
    abilities: &mut HashMap<usize, Ability>,
) {
    abilities.remove(piste_id);

    let Some(piste) = pistes.get(piste_id) else {
        return;
    };

    let Some(costs) = costs.get(piste_id) else {
        return;
    };

    let Some(exits) = exits.get(piste_id) else {
        return;
    };

    if let Some(ability) = compute_ability(piste_id, piste, costs, exits, lifts, gates) {
        abilities.insert(*piste_id, ability);
    }
}

fn compute_ability(
    piste_id: &usize,
    piste: &Piste,
    costs: &Costs<State>,
    exits: &[Exit],
    lifts: &HashMap<usize, Lift>,
    gates: &HashMap<usize, Gate>,
) -> Option<Ability> {
    let grid = &piste.grid;

    let lifts_iter = lifts.values().map(|lift| lift.drop_off.state);

    let gates_iter = gates
        .values()
        .filter(|gate| gate.destination_piste == *piste_id)
        .flat_map(|gate| gate.footprint.iter())
        .map(|position| State {
            position,
            velocity: 0,
            travel_direction: Direction::North,
        });

    let gates = lifts_iter
        .chain(gates_iter)
        .filter(|State { position, .. }| grid.in_bounds(position) && grid[position])
        .map(|state| State {
            velocity: 0,
            ..state
        })
        .collect::<HashSet<_>>();

    let targets = exits.iter().map(|exit| exit.id).collect::<HashSet<_>>();

    let mut abilities = Vec::with_capacity(gates.len() * targets.len());
    for gate in gates.iter() {
        for target in targets.iter() {
            if let Some(ability) = costs.min_ability(gate, target) {
                abilities.push(ability);
            }
        }
    }

    abilities.iter().max().copied()
}
