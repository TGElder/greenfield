use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::iter::empty;

use commons::grid::{Grid, CORNERS_INVERSE};
use network::model::{Edge, OutNetwork};

use crate::model::ability::Ability;
use crate::model::direction::Direction;
use crate::model::entrance::Entrance;
use crate::model::lift::Lift;
use crate::model::piste::Costs;
use crate::model::skiing::State;

pub struct PisteNetwork<'a> {
    pub piste_map: &'a Grid<Option<usize>>,
    pub lifts: &'a HashMap<usize, Lift>,
    pub entrances: &'a HashMap<usize, Entrance>,
    pub costs: &'a HashMap<usize, Costs>,
    pub ability: Ability,
}

impl<'a> OutNetwork<usize> for PisteNetwork<'a> {
    fn edges_out<'b>(
        &'b self,
        from: &'b usize,
    ) -> Box<dyn Iterator<Item = network::model::Edge<usize>> + 'b> {
        let Some(positions) = self
            .lifts
            .get(from)
            .map(|lift| vec![lift.drop_off.position])
            .or_else(|| {
                self.entrances
                    .get(from)
                    .map(|entrance| entrance.footprint.iter().collect::<Vec<_>>())
            })
        else {
            return Box::new(empty());
        };

        let mut targets_to_costs_vec = HashMap::<usize, Vec<u64>>::new();
        for position in positions.iter() {
            let pistes = self
                .piste_map
                .offsets(position, &CORNERS_INVERSE)
                .flat_map(|corner| self.piste_map[corner])
                .collect::<HashSet<_>>();
            let mut target_to_costs = HashMap::<usize, u64>::new();
            for piste in pistes {
                let Some(costs) = self.costs.get(&piste) else {
                    continue;
                };
                let state = State {
                    position: *position,
                    velocity: 0,
                    travel_direction: Direction::North,
                };
                let targets = costs
                    .targets_reachable_from_state(&state, &self.ability)
                    .collect::<HashSet<_>>();
                for target in targets {
                    let cost = costs.costs(*target, self.ability).unwrap()[&state];
                    match target_to_costs.entry(*target) {
                        Entry::Occupied(mut value) => {
                            let value = value.get_mut();
                            *value = cost.max(*value);
                        }
                        Entry::Vacant(cell) => {
                            cell.insert(cost);
                        }
                    }
                }
            }
            for (target, cost) in target_to_costs {
                targets_to_costs_vec
                    .entry(target)
                    .or_insert_with(|| Vec::with_capacity(positions.len()))
                    .push(cost);
            }
        }

        let position_count = positions.len();

        let iter = targets_to_costs_vec
            .into_iter()
            .filter(move |(_, costs)| costs.len() == position_count)
            .map(|(to, costs)| Edge {
                from: *from,
                to,
                cost: costs.into_iter().max().unwrap().try_into().unwrap(),
            });

        Box::new(iter)

        // If lift, find all pistes at drop off
        // If entrance, we know the piste

        // Now we have (piste + positions)

        // Find targets accessible from all positions

        // Target setter should run over all pistes? Piste adopotion as part of target setting
    }
}
