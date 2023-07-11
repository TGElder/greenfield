use std::collections::{HashMap, HashSet};
use std::iter::empty;
use std::time::Duration;

use commons::geometry::XY;
use commons::grid::Grid;
use network::model::{Edge, InNetwork, OutNetwork};

use crate::model::skiing::{Mode, State};
use crate::model::DIRECTIONS;

const WALK_DURATION: Duration = Duration::from_secs(1);
const SKIS_OFF_DURATION: Duration = Duration::from_secs(10);

pub struct WalkingNetwork<'a> {
    pub terrain: &'a Grid<f32>,
    pub reserved: &'a Grid<bool>,
}

impl<'a> OutNetwork<State> for WalkingNetwork<'a> {
    fn edges_out<'b>(
        &'b self,
        from: &'b State,
    ) -> Box<dyn Iterator<Item = ::network::model::Edge<State>> + 'b> {
        Box::new(self.skis_off(from).chain(self.walk(from)))
    }
}

impl<'a> WalkingNetwork<'a> {
    fn skis_off(
        &'a self,
        from: &'a State,
    ) -> impl Iterator<Item = ::network::model::Edge<State>> + 'a {
        match from {
            State {
                mode: Mode::Skiing { velocity },
                ..
            } if *velocity == 0 => Some(from),
            _ => None,
        }
        .into_iter()
        .map(|from| Edge {
            from: *from,
            to: State {
                mode: Mode::Walking,
                ..*from
            },
            cost: SKIS_OFF_DURATION.as_micros().try_into().unwrap(),
        })
    }

    fn walk(&'a self, from: &'a State) -> impl Iterator<Item = ::network::model::Edge<State>> + 'a {
        match from {
            State {
                mode: Mode::Walking,
                ..
            } => Some(from),
            _ => None,
        }
        .into_iter()
        .flat_map(|from| {
            self.terrain
                .neighbours_4(from.position)
                .filter(|neighbour| !self.reserved[neighbour])
                .map(|neighbour| Edge {
                    from: *from,
                    to: State {
                        position: neighbour,
                        mode: Mode::Walking,
                        ..*from
                    },
                    cost: WALK_DURATION.as_micros().try_into().unwrap(),
                })
        })
    }
}

pub struct WalkingInNetwork {
    pub edges: HashMap<State, Vec<Edge<State>>>,
}

impl WalkingInNetwork {
    pub fn for_positions(
        network: &dyn OutNetwork<State>,
        positions: &HashSet<XY<u32>>,
    ) -> WalkingInNetwork {
        let mut edges = HashMap::with_capacity(positions.len());

        for position in positions {
            for travel_direction in DIRECTIONS {
                let walking = State {
                    position: *position,
                    mode: Mode::Walking,
                    travel_direction,
                };

                for edge in network
                    .edges_out(&walking)
                    .filter(|Edge { to, .. }| positions.contains(&to.position))
                {
                    edges
                        .entry(edge.to)
                        .or_insert_with(|| Vec::with_capacity(5))
                        .push(edge);
                }

                let state = State {
                    position: *position,
                    mode: Mode::Skiing { velocity: 0 },
                    travel_direction,
                };

                for edge in network
                    .edges_out(&state)
                    .filter(|Edge { to, .. }| positions.contains(&to.position))
                {
                    edges
                        .entry(edge.to)
                        .or_insert_with(|| Vec::with_capacity(5))
                        .push(edge);
                }
            }
        }

        WalkingInNetwork { edges }
    }
}

impl InNetwork<State> for WalkingInNetwork {
    fn edges_in<'a>(&'a self, to: &'a State) -> Box<dyn Iterator<Item = Edge<State>> + 'a> {
        match self.edges.get(to) {
            Some(edges) => Box::new(edges.iter().copied()),
            None => Box::new(empty()),
        }
    }
}
