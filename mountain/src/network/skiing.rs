use std::collections::{HashMap, HashSet};
use std::iter::empty;
use std::time::Duration;

use commons::{geometry::XY, grid::Grid};
use network::model::{Edge, InNetwork, OutNetwork};

use crate::model::skiing::{Mode, State};
use crate::model::DIRECTIONS;
use crate::network::velocity_encoding::VELOCITY_LEVELS;
use crate::{
    model::Direction,
    network::velocity_encoding::{decode_velocity, encode_velocity},
    physics,
};

const TURNING_DURATION: Duration = Duration::from_secs(1);
const SKIS_ON_DURATION: Duration = Duration::from_secs(10);

pub struct SkiingNetwork<'a> {
    pub terrain: &'a Grid<f32>,
    pub reserved: &'a Grid<bool>,
}

impl<'a> OutNetwork<State> for SkiingNetwork<'a> {
    fn edges_out<'b>(
        &'b self,
        from: &'b State,
    ) -> Box<dyn Iterator<Item = ::network::model::Edge<State>> + 'b> {
        Box::new(
            self.skiing_edges(from)
                .chain(self.braking_edges(from))
                .chain(self.turning_edges(from))
                .chain(self.skis_on(from)),
        )
    }
}

impl<'a> SkiingNetwork<'a> {
    fn skiing_edges(
        &'a self,
        from: &'a State,
    ) -> impl Iterator<Item = ::network::model::Edge<State>> + 'a {
        match from {
            State {
                mode: Mode::Skiing { velocity, .. },
                ..
            } => Some(velocity),
            _ => None,
        }
        .into_iter()
        .flat_map(|velocity| {
            [
                from.travel_direction.next_anticlockwise(),
                from.travel_direction,
                from.travel_direction.next_clockwise(),
            ]
            .into_iter()
            .flat_map(|travel_direction| {
                self.get_skiing_edge(from, travel_direction, *velocity, 0.0)
            })
        })
    }

    fn get_skiing_edge(
        &self,
        from: &State,
        travel_direction: Direction,
        velocity: u8,
        friction: f32,
    ) -> Option<Edge<State>> {
        let to_position = self.get_to_position(&from.position, &travel_direction)?;

        if self.reserved[to_position] {
            return None;
        }

        let initial_velocity: f32 = decode_velocity(&velocity)?;

        let run = travel_direction.run();
        let rise = self.terrain[to_position] - self.terrain[from.position];
        let physics::skiing::Solution { velocity, duration } =
            physics::skiing::solve(initial_velocity, run, rise, friction)?;

        Some(Edge {
            from: *from,
            to: State {
                position: to_position,
                mode: Mode::Skiing {
                    velocity: encode_velocity(&velocity)?,
                },
                travel_direction,
            },
            cost: (duration * 1_000_000.0).round() as u32,
        })
    }

    fn braking_edges(
        &'a self,
        from: &'a State,
    ) -> impl Iterator<Item = ::network::model::Edge<State>> + 'a {
        match from {
            State {
                mode: Mode::Skiing { velocity, .. },
                ..
            } => Some(velocity),
            _ => None,
        }
        .into_iter()
        .flat_map(move |velocity| {
            self.get_skiing_edge(from, from.travel_direction, *velocity, 1.0)
                .into_iter()
                .flat_map(move |edge| {
                    [
                        from.travel_direction.next_anticlockwise(),
                        from.travel_direction.next_clockwise(),
                    ]
                    .into_iter()
                    .map(move |to_direction| Edge {
                        to: State {
                            travel_direction: to_direction,
                            ..edge.to
                        },
                        ..edge
                    })
                })
        })
    }

    fn turning_edges(
        &'a self,
        from: &'a State,
    ) -> impl Iterator<Item = ::network::model::Edge<State>> + 'a {
        match from {
            State {
                mode: Mode::Skiing { velocity, .. },
                ..
            } if *velocity == 0 => Some(from),
            _ => None,
        }
        .into_iter()
        .flat_map(|from| {
            let turning_micros = TURNING_DURATION.as_micros().try_into().unwrap();
            [
                Edge {
                    from: *from,
                    to: State {
                        travel_direction: from.travel_direction.next_clockwise(),
                        ..*from
                    },
                    cost: turning_micros,
                },
                Edge {
                    from: *from,
                    to: State {
                        travel_direction: from.travel_direction.next_anticlockwise(),
                        ..*from
                    },
                    cost: turning_micros,
                },
            ]
            .into_iter()
        })
    }

    fn skis_on(
        &'a self,
        from: &'a State,
    ) -> impl Iterator<Item = ::network::model::Edge<State>> + 'a {
        match from {
            State {
                mode: Mode::Walking,
                ..
            } => Some(from),
            _ => None,
        }
        .into_iter()
        .map(|from| Edge {
            from: *from,
            to: State {
                mode: Mode::Skiing { velocity: 0 },
                ..*from
            },
            cost: SKIS_ON_DURATION.as_micros().try_into().unwrap(),
        })
    }

    fn get_to_position(&self, position: &XY<u32>, travel_direction: &Direction) -> Option<XY<u32>> {
        let offset = travel_direction.offset();
        self.terrain.offset(position, offset)
    }
}

pub struct SkiingInNetwork {
    pub edges: HashMap<State, Vec<Edge<State>>>,
}

impl SkiingInNetwork {
    pub fn for_positions(
        network: &dyn OutNetwork<State>,
        positions: &HashSet<XY<u32>>,
    ) -> SkiingInNetwork {
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

                for velocity in 0..VELOCITY_LEVELS {
                    let state = State {
                        position: *position,
                        mode: Mode::Skiing { velocity },
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
        }

        SkiingInNetwork { edges }
    }
}

impl InNetwork<State> for SkiingInNetwork {
    fn edges_in<'a>(&'a self, to: &'a State) -> Box<dyn Iterator<Item = Edge<State>> + 'a> {
        match self.edges.get(to) {
            Some(edges) => Box::new(edges.iter().copied()),
            None => Box::new(empty()),
        }
    }
}
