use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::iter::empty;

use commons::geometry::XY;
use commons::grid::{Grid, CORNERS_INVERSE};
use network::model::{Edge, OutNetwork};

use crate::model::ability::Ability;
use crate::model::carousel::Carousel;
use crate::model::direction::DIRECTIONS;
use crate::model::entrance::Entrance;
use crate::model::lift::Lift;
use crate::model::piste::Costs;
use crate::model::skiing::State;

pub struct TargetNetwork<'a> {
    pub piste_map: &'a Grid<Option<usize>>,
    pub lifts: &'a HashMap<usize, Lift>,
    pub carousels: &'a HashMap<usize, Carousel>,
    pub entrances: &'a HashMap<usize, Entrance>,
    pub costs: &'a HashMap<usize, Costs>,
    pub abilities: &'a HashMap<usize, Ability>,
    pub ability: Ability,
}

impl<'a> TargetNetwork<'a> {
    fn get_from_states(&self, from: &usize) -> Option<Vec<State>> {
        self.lift_states(from)
            .or_else(|| self.entrance_states(from))
    }

    fn lift_states(&self, target: &usize) -> Option<Vec<State>> {
        self.lifts.get(target).map(|lift| {
            vec![State {
                velocity: 0,
                ..lift.drop_off.state
            }]
        })
    }

    fn entrance_states(&self, target: &usize) -> Option<Vec<State>> {
        self.entrances.get(target).map(states_for_entrance)
    }

    fn pistes_at_positon(&self, position: &XY<u32>) -> HashSet<usize> {
        self.piste_map
            .offsets(position, &CORNERS_INVERSE)
            .flat_map(|corner| self.piste_map[corner])
            .filter(|piste_id| self.abilities[piste_id] <= self.ability)
            .collect::<HashSet<_>>()
    }

    fn lift_travel_micros(&self, lift: &usize) -> u64 {
        self.lifts
            .get(lift)
            .and_then(|lift| {
                self.carousels
                    .get(&lift.carousel_id)
                    .map(|carousel| lift.ride_length_meters() / carousel.velocity)
            })
            .map(|seconds| seconds * 1_000_000.0)
            .map(|micros| micros as u64)
            .unwrap_or(0)
    }
}

fn states_for_entrance(entrance: &Entrance) -> Vec<State> {
    entrance
        .footprint
        .iter()
        .flat_map(stationary_states_for_position)
        .collect()
}

fn stationary_states_for_position(position: XY<u32>) -> impl Iterator<Item = State> {
    DIRECTIONS.into_iter().map(move |travel_direction| State {
        position,
        travel_direction,
        velocity: 0,
    })
}

// this gets the other lifts or entrances reachable from a given lift or entrance
impl<'a> OutNetwork<usize> for TargetNetwork<'a> {
    fn edges_out<'b>(
        &'b self,
        from: &'b usize,
    ) -> Box<dyn Iterator<Item = network::model::Edge<usize>> + 'b> {
        // from node (a lift drop off or entrance) may have multiple states
        let Some(from_states) = self.get_from_states(from) else {
            return Box::new(empty());
        };

        // we form a map of targets reachable from any from state
        // the vector contains a cost for each from state from which that target can be reached
        let mut targets_to_costs =
            HashMap::<usize, Vec<u64>>::with_capacity(self.lifts.len() + self.entrances.len());
        for from_state in from_states.iter() {
            let pistes = self.pistes_at_positon(&from_state.position);

            let mut target_to_cost = HashMap::<usize, u64>::new();

            // a from state can exist in multiple pistes so we need to check what is reachable on each piste
            for piste in pistes {
                let Some(costs) = self.costs.get(&piste) else {
                    continue;
                };
                costs
                    .targets_reachable_from_state(from_state, &self.ability)
                    .filter(|&(target, _)| target != from)
                    .for_each(|(&target, &cost)| match target_to_cost.entry(target) {
                        Entry::Occupied(mut value) => {
                            let value = value.get_mut();
                            *value = cost.max(*value); // if the target is reachable from multiple pistes we take the max cost
                        }
                        Entry::Vacant(cell) => {
                            cell.insert(cost);
                        }
                    });
            }

            // we want a single cost per reachable target which is why we add to the vector outside the pistes loop
            for (target, cost) in target_to_cost {
                targets_to_costs
                    .entry(target)
                    .or_insert_with(|| Vec::with_capacity(from_states.len()))
                    .push(cost);
            }
        }

        let from_state_count = from_states.len();
        let lift_travel_time = self.lift_travel_micros(from);
        let edges = targets_to_costs
            .into_iter()
            .filter(move |(_, costs)| costs.len() == from_state_count) // we only want targets reachable by all from states
            .map(move |(to, costs)| {
                let cost = costs.into_iter().max().unwrap();
                Edge {
                    from: *from,
                    to,
                    cost: ((lift_travel_time + cost) / 1000).try_into().unwrap(), // divide by 1000 to avoid exceeding u32 limit
                }
            });

        Box::new(edges)
    }
}