use std::collections::HashMap;

use commons::grid::Grid;
use network::model::OutNetwork;

use crate::model::ability::Ability;
use crate::model::entrance::Entrance;
use crate::model::lift::Lift;
use crate::model::piste::Costs;
use crate::network::piste::PisteNetwork;

pub fn compute_piste_network(
    piste_map: &Grid<Option<usize>>,
    lifts: &HashMap<usize, Lift>,
    entrances: &HashMap<usize, Entrance>,
    costs: &HashMap<usize, Costs>,
    ability: Ability,
) {
    let network = PisteNetwork {
        piste_map,
        lifts,
        entrances,
        costs,
        ability,
    };

    for entrance in entrances.keys() {
        println!("Entrance {:?}", entrance);
        for edge in network.edges_out(entrance) {
            println!("  {:?}", edge);
        }
    }

    for lift in lifts.keys() {
        println!("Lift {:?}", lift);
        for edge in network.edges_out(lift) {
            println!("  {:?}", edge);
        }
    }
}
