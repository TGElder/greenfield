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
    exits: &HashMap<usize, Exit>,
    abilities: &mut HashMap<usize, Ability>,
) {
    abilities.remove(piste_id);

    let Some(costs) = costs.get(piste_id) else {
        return;
    };

    let entrances = entrances
        .values()
        .filter(
            |Entrance {
                 destination_piste_id,
                 ..
             }| destination_piste_id == piste_id,
        )
        .collect::<Vec<_>>();

    let exit_ids = exits
        .iter()
        .filter(
            |(
                _,
                Exit {
                    origin_piste_id, ..
                },
            )| origin_piste_id == piste_id,
        )
        .map(|(exit_id, _)| exit_id)
        .collect::<Vec<_>>();

    if let Some(ability) = compute_ability(costs, &entrances, &exit_ids) {
        abilities.insert(*piste_id, ability);
    }
}

fn compute_ability(
    costs: &Costs<State>,
    entrances: &[&Entrance],
    exit_ids: &[&usize],
) -> Option<Ability> {
    entrances
        .iter()
        .flat_map(|entrance| &entrance.stationary_states)
        .flat_map(|state| {
            exit_ids
                .iter()
                .filter_map(|exit_id| costs.min_ability(state, exit_id))
        })
        .max()
}
