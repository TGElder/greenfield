use commons::grid::Grid;
use network::algorithms::costs_to_targets::CostsToTargets;
use network::utils::MaterializedInNetwork;
use std::collections::{HashMap, HashSet};

use crate::model::ability::{Ability, ABILITIES};
use crate::model::carousel::Carousel;
use crate::model::costs::Costs;
use crate::model::entrance::Entrance;
use crate::model::lift::Lift;
use crate::model::skiing::State;
use crate::network::global::GlobalNetwork;

pub struct Parameters<'a> {
    pub piste_map: &'a Grid<Option<usize>>,
    pub lifts: &'a HashMap<usize, Lift>,
    pub carousels: &'a HashMap<usize, Carousel>,
    pub entrances: &'a HashMap<usize, Entrance>,
    pub costs: &'a HashMap<usize, Costs<State>>,
    pub abilities: &'a HashMap<usize, Ability>,
    pub open: &'a HashSet<usize>,
    pub global_costs: &'a mut Costs<usize>,
}

pub fn compute_global_costs(
    Parameters {
        piste_map,
        lifts,
        carousels,
        entrances,
        costs,
        abilities,
        open,
        global_costs,
    }: Parameters<'_>,
) {
    *global_costs = Costs::new();

    let lift_exit_to_lift = &lifts
        .iter()
        .map(|(lift_id, Lift { exit_id, .. })| (*exit_id, *lift_id))
        .collect::<HashMap<_, _>>();

    for ability in ABILITIES {
        let network = GlobalNetwork {
            piste_map,
            lifts,
            lift_exit_to_lift,
            carousels,
            entrances,
            costs,
            abilities,
            ability,
        };

        let targets = entrances
            .iter()
            .filter(|&(target, _)| open.contains(target))
            .filter(
                |&(
                    _,
                    Entrance {
                        destination_piste_id,
                        ..
                    },
                )| {
                    abilities
                        .get(destination_piste_id)
                        .map(|&piste_ability| piste_ability <= ability)
                        .unwrap_or_default()
                },
            )
            .map(|(target, _)| target)
            .copied()
            .collect::<HashSet<_>>();

        let network = MaterializedInNetwork::from_out_network(&network, &targets);

        for target in targets.iter() {
            let costs = network.costs_to_targets(&HashSet::from([*target]), None);
            global_costs.set_costs(*target, ability, costs)
        }
    }
}
