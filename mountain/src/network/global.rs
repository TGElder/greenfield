use std::collections::HashMap;
use std::iter::{empty, once};

use network::model::{Edge, OutNetwork};

use crate::model::ability::Ability;
use crate::model::carousel::Carousel;
use crate::model::costs::Costs;
use crate::model::entrance::Entrance;
use crate::model::lift::Lift;
use crate::model::skiing::State;

pub const GLOBAL_COST_DIVISOR: u64 = 1000;

pub struct GlobalNetwork<'a> {
    pub lifts: &'a HashMap<usize, Lift>,
    pub pick_up_to_lift: &'a HashMap<usize, usize>,
    pub carousels: &'a HashMap<usize, Carousel>,
    pub entrances: &'a HashMap<usize, Entrance>,
    pub costs: &'a HashMap<usize, Costs<State>>,
    pub abilities: &'a HashMap<usize, Ability>,
    pub ability: Ability,
}

impl<'a> GlobalNetwork<'a> {
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
            .map(|micros| micros / GLOBAL_COST_DIVISOR) // to avoid exceeding u32 limit
            .unwrap_or(0)
    }
}

impl<'a> OutNetwork<usize> for GlobalNetwork<'a> {
    fn edges_out<'b>(
        &'b self,
        from: &'b usize,
    ) -> Box<dyn Iterator<Item = network::model::Edge<usize>> + 'b> {
        // lift edge
        if let Some(lift_id) = self.pick_up_to_lift.get(from) {
            return Box::new(once(Edge {
                from: *from,
                to: self.lifts[lift_id].drop_off.id,
                cost: self.lift_travel_micros(from).try_into().unwrap(),
            }));
        }

        let Some(Entrance {
            destination_piste_id: piste_id,
            stationary_states: from_states,
            ..
        }) = self.entrances.get(from)
        else {
            return Box::new(empty());
        };

        let Some(&piste_ability) = self.abilities.get(piste_id) else {
            return Box::new(empty());
        };
        if piste_ability > self.ability {
            return Box::new(empty());
        }

        // we form a map of targets reachable from any from state
        // the vector contains a cost for each from state from which that target can be reached
        let mut target_to_costs = HashMap::<usize, Vec<u64>>::with_capacity(self.entrances.len());

        for from_state in from_states {
            let Some(costs) = self.costs.get(piste_id) else {
                continue;
            };
            costs
                .targets_reachable_from_node(from_state, &self.ability)
                .filter(|&(target, _)| target != from)
                .for_each(|(&target, &cost)| {
                    target_to_costs
                        .entry(target)
                        .or_insert_with(|| Vec::with_capacity(from_states.len()))
                        .push(cost)
                });
        }

        let edges = target_to_costs
            .into_iter()
            .filter(move |(_, costs)| costs.len() == from_states.len()) // we only want targets reachable by all from states
            .map(move |(to, costs)| {
                let cost = costs.into_iter().max().unwrap();
                Edge {
                    from: *from,
                    to,
                    cost: (cost / GLOBAL_COST_DIVISOR).try_into().unwrap(), // to avoid exceeding u32 limit
                }
            });

        Box::new(edges)
    }
}
