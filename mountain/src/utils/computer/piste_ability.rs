use std::collections::HashMap;

use crate::model::ability::Ability;
use crate::model::costs::Costs;
use crate::model::entrance::Entrance;
use crate::model::exit::Exit;
use crate::model::skiing::State;

pub fn compute_piste(
    piste_id: &usize,
    costs: &HashMap<usize, Costs<State>>,
    entrances: &HashMap<usize, Entrance>,
    exits: &HashMap<usize, Vec<Exit>>,
    abilities: &mut HashMap<usize, Ability>,
) {
    abilities.remove(piste_id);

    let Some(costs) = costs.get(piste_id) else {
        return;
    };

    let Some(exits) = exits.get(piste_id) else {
        return;
    };

    if let Some(ability) = compute_ability(piste_id, costs, entrances, exits) {
        abilities.insert(*piste_id, ability);
    }
}

fn compute_ability(
    piste_id: &usize,
    costs: &Costs<State>,
    entrances: &HashMap<usize, Entrance>,
    exits: &[Exit],
) -> Option<Ability> {
    entrances
        .values()
        .filter(
            |Entrance {
                 destination_piste_id,
                 ..
             }| destination_piste_id == piste_id,
        )
        .flat_map(
            |Entrance {
                 stationary_states: states,
                 ..
             }| states,
        )
        .flat_map(|state| {
            exits
                .iter()
                .map(|exit| &exit.id)
                .filter_map(|target| costs.min_ability(state, target))
        })
        .max()
}
