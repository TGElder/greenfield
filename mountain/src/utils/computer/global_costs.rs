use commons::map::ContainsKeyValue;
use network::algorithms::costs_to_targets::CostsToTargets;
use network::utils::MaterializedInNetwork;
use std::collections::{HashMap, HashSet};

use crate::model::ability::{Ability, ABILITIES};
use crate::model::carousel::Carousel;
use crate::model::costs::Costs;
use crate::model::entrance::Entrance;
use crate::model::exit::Exit;
use crate::model::lift::Lift;
use crate::model::skiing::State;
use crate::network::global::GlobalNetwork;

pub struct Parameters<'a> {
    pub lifts: &'a HashMap<usize, Lift>,
    pub carousels: &'a HashMap<usize, Carousel>,
    pub entrances: &'a HashMap<usize, Entrance>,
    pub exits: &'a HashMap<usize, Exit>,
    pub costs: &'a HashMap<usize, Costs<State>>,
    pub abilities: &'a HashMap<usize, Ability>,
    pub open: &'a HashMap<usize, bool>,
    pub global_costs: &'a mut Costs<usize>,
}

pub fn compute_global_costs(
    Parameters {
        lifts,
        carousels,
        entrances,
        exits,
        costs,
        abilities,
        open,
        global_costs,
    }: Parameters<'_>,
) {
    *global_costs = Costs::new();

    let pick_up_to_lift = &lifts
        .iter()
        .map(|(lift_id, Lift { pick_up, .. })| (pick_up.id, *lift_id))
        .collect::<HashMap<_, _>>();

    for ability in ABILITIES {
        let network = GlobalNetwork {
            lifts,
            pick_up_to_lift,
            carousels,
            entrances,
            open,
            costs,
            abilities,
            ability,
        };

        let targets = entrances
            .keys()
            .chain(exits.keys())
            .filter(|&target| open.contains_key_value(target, true))
            .copied()
            .collect::<HashSet<_>>();

        let network = MaterializedInNetwork::from_out_network(&network, &targets);

        for target in targets.iter() {
            let costs = network.costs_to_targets(&HashSet::from([*target]), None);
            global_costs.set_costs(*target, ability, costs)
        }
    }
}
