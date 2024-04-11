use std::collections::{HashMap, HashSet};

use commons::grid::Grid;
use network::algorithms::costs_to_targets::CostsToTargets;

use crate::model::ability::ABILITIES;
use crate::model::entrance::Entrance;
use crate::model::lift::Lift;
use crate::model::piste::Costs;
use crate::model::skiing::State;
use crate::network::piste::{PisteInNetwork, PisteOutNetwork};

pub fn compute_piste_network(
    piste_map: &Grid<Option<usize>>,
    lifts: &HashMap<usize, Lift>,
    entrances: &HashMap<usize, Entrance>,
    costs: &HashMap<usize, Costs<State>>,
    piste_costs: &mut Costs<usize>,
) {
    *piste_costs = Costs::new();

    let targets = lifts
        .keys()
        .chain(entrances.keys())
        .copied()
        .collect::<Vec<_>>();

    for ability in ABILITIES {
        let network = PisteOutNetwork {
            piste_map,
            lifts,
            entrances,
            costs,
            ability,
        };

        let network = PisteInNetwork::new(&network, &targets);

        for target in targets.iter() {
            let costs = network.costs_to_targets(&HashSet::from([*target]), None);
            piste_costs.set_costs(*target, ability, costs)
        }
    }
}
