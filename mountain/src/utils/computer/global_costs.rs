use std::collections::{HashMap, HashSet};

use commons::grid::Grid;
use network::algorithms::costs_to_targets::CostsToTargets;
use network::utils::MaterializedInNetwork;

use crate::model::ability::{Ability, ABILITIES};
use crate::model::carousel::Carousel;
use crate::model::costs::Costs;
use crate::model::entrance::Entrance;
use crate::model::lift::Lift;
use crate::model::skiing::State;
use crate::network::global::GlobalNetwork;

pub fn compute_global_costs(
    piste_map: &Grid<Option<usize>>,
    lifts: &HashMap<usize, Lift>,
    carousels: &HashMap<usize, Carousel>,
    entrances: &HashMap<usize, Entrance>,
    costs: &HashMap<usize, Costs<State>>,
    abilities: &HashMap<usize, Ability>,
    global_costs: &mut Costs<usize>,
) {
    *global_costs = Costs::new();

    let targets = lifts
        .keys()
        .chain(entrances.keys())
        .copied()
        .collect::<HashSet<_>>();

    for ability in ABILITIES {
        let network = GlobalNetwork {
            piste_map,
            lifts,
            carousels,
            entrances,
            costs,
            abilities,
            ability,
        };

        let network = MaterializedInNetwork::from_out_network(&network, &targets);

        for target in targets.iter() {
            let costs = network.costs_to_targets(&HashSet::from([*target]), None);
            global_costs.set_costs(*target, ability, costs)
        }
    }
}
