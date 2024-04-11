use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::iter::empty;

use commons::grid::{Grid, CORNERS_INVERSE};
use network::model::{Edge, InNetwork, OutNetwork};

use crate::handlers::lift_builder::LIFT_VELOCITY;
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
                    .copied()
                    .filter(|target| target != from)
                    .collect::<HashSet<_>>();
                for target in targets {
                    let cost = costs.costs(target, self.ability).unwrap()[&state];
                    match target_to_costs.entry(target) {
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

        let lift_cost = self
            .lifts
            .get(from)
            .map(|lift| {
                println!("Ride length = {}", lift.ride_length());
                lift.ride_length() / LIFT_VELOCITY
            }) // TODO lookup from carousel
            .map(|seconds| seconds * 1_000_000.0)
            .map(|micros| micros as u32)
            .unwrap_or(0);

        println!("Lift cost = {:?}", lift_cost);

        let iter = targets_to_costs_vec
            .into_iter()
            .filter(move |(_, costs)| costs.len() == position_count)
            .map(move |(to, costs)| {
                let cost: u32 = costs.into_iter().max().unwrap().try_into().unwrap();
                Edge {
                    from: *from,
                    to,
                    cost: (lift_cost + cost) / 1000,
                }
            });

        Box::new(iter)
    }
}

pub struct PisteInNetwork {
    // TODO make generic helper in network crate?
    pub edges: HashMap<usize, Vec<Edge<usize>>>,
}

impl PisteInNetwork {
    pub fn new(network: &dyn OutNetwork<usize>, nodes: &[usize]) -> PisteInNetwork {
        let mut edges = HashMap::with_capacity(nodes.len());

        for node in nodes {
            for edge in network.edges_out(node) {
                edges.entry(edge.to).or_insert_with(Vec::new).push(edge);
            }
        }

        PisteInNetwork { edges }
    }
}

impl InNetwork<usize> for PisteInNetwork {
    fn edges_in<'a>(&'a self, to: &'a usize) -> Box<dyn Iterator<Item = Edge<usize>> + 'a> {
        match self.edges.get(to) {
            Some(edges) => Box::new(edges.iter().copied()),
            None => Box::new(empty()),
        }
    }
}