use std::collections::{HashMap, HashSet};

use commons::grid::Grid;

use crate::model::ability::Ability;
use crate::model::costs::Costs;
use crate::model::entrance::Entrance;
use crate::model::exit::Exit;
use crate::model::piste::Piste;
use crate::model::reservation::Reservation;
use crate::model::skiing::State;
use crate::systems::global_computer;
use crate::utils::computer;

pub struct System {
    pistes_to_compute: HashSet<usize>,
}

pub struct Parameters<'a> {
    pub pistes: &'a HashMap<usize, Piste>,
    pub entrances: &'a HashMap<usize, Entrance>,
    pub exits: &'a HashMap<usize, Exit>,
    pub terrain: &'a Grid<f32>,
    pub reservations: &'a Grid<HashMap<usize, Reservation>>,
    pub costs: &'a mut HashMap<usize, Costs<State>>,
    pub abilities: &'a mut HashMap<usize, Ability>,
    pub global_computer: &'a mut global_computer::System,
}

impl System {
    pub fn new() -> System {
        System {
            pistes_to_compute: HashSet::default(),
        }
    }

    pub fn compute(&mut self, piste: usize) {
        self.pistes_to_compute.insert(piste);
    }

    pub fn run(&mut self, mut parameters: Parameters) {
        if self.pistes_to_compute.is_empty() {
            return;
        }

        for id in self.pistes_to_compute.drain() {
            recompute_piste(&id, &mut parameters);
        }

        parameters.global_computer.update();
    }
}

fn recompute_piste(
    id: &usize,
    Parameters {
        pistes,
        entrances,
        exits,
        terrain,
        reservations,
        abilities,
        costs,
        ..
    }: &mut Parameters<'_>,
) {
    computer::costs::compute_piste(id, pistes, terrain, exits, reservations, costs);
    computer::piste_ability::compute_piste(id, costs, entrances, exits, abilities);
}
