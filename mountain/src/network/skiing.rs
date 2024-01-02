use std::collections::{HashMap, HashSet};
use std::iter::{empty, once};
use std::time::Duration;

use commons::{geometry::XY, grid::Grid};
use network::model::{Edge, InNetwork, OutNetwork};

use crate::model::direction::{Direction, DIRECTIONS};
use crate::model::skiing::{Mode, State};
use crate::network::velocity_encoding::VELOCITY_LEVELS;
use crate::{
    network::velocity_encoding::{decode_velocity, encode_velocity},
    utils::physics,
};

const TURNING_DURATION: Duration = Duration::from_secs(1);
const SKIS_ON_DURATION: Duration = Duration::from_secs(10);
const SKIS_OFF_DURATION: Duration = Duration::from_secs(10);

const BRAKING_FRICTION: f32 = 1.0;

const POLING_ACCELERATION: f32 = 2.5;
const POLING_MAX_VELOCITY: f32 = 2.0;

const STOP_MAX_VELOCITY: f32 = 1.5;

pub struct SkiingNetwork<'a> {
    pub terrain: &'a Grid<f32>,
    pub is_reserved_fn: &'a dyn Fn(&XY<u32>) -> bool,
    pub is_skiable_edge_fn: &'a dyn Fn(&State, &State) -> bool,
}

impl<'a> OutNetwork<State> for SkiingNetwork<'a> {
    fn edges_out<'b>(
        &'b self,
        from: &'b State,
    ) -> Box<dyn Iterator<Item = ::network::model::Edge<State>> + 'b> {
        Box::new(
            self.poling_edges(from)
                .chain(self.skiing_edges(from))
                .chain(self.braking_edges(from))
                .filter(|edge| (self.is_skiable_edge_fn)(&edge.to, &edge.from))
                .chain(self.turning_edges(from))
                .chain(self.stop_edge(from))
                .chain(self.skis_off(from))
                .chain(self.skis_on(from)),
        )
    }
}

impl<'a> SkiingNetwork<'a> {
    fn skiing_edges(
        &'a self,
        from: &'a State,
    ) -> impl Iterator<Item = ::network::model::Edge<State>> + 'a {
        once(from)
            .flat_map(get_skiing_velocity)
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

        if (self.is_reserved_fn)(&to_position) {
            return None;
        }

        let initial_velocity: f32 = decode_velocity(&velocity)?;

        let run = travel_direction.run();
        let rise = self.terrain[to_position] - self.terrain[from.position];
        let physics::skiing::Solution { velocity, duration } =
            physics::skiing::solve(initial_velocity, run, rise, 0.0, friction)?;

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
        once(from)
            .flat_map(get_skiing_velocity)
            .flat_map(move |velocity| {
                self.get_skiing_edge(from, from.travel_direction, *velocity, BRAKING_FRICTION)
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
        once(from)
            .filter(|from| from.mode == Mode::Skiing { velocity: 0 })
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

    fn stop_edge(
        &'a self,
        from: &'a State,
    ) -> impl Iterator<Item = ::network::model::Edge<State>> + 'a {
        let max_velocity_encoded = encode_velocity(&STOP_MAX_VELOCITY).unwrap();

        once(from)
            .flat_map(get_skiing_velocity)
            .filter(move |velocity| **velocity <= max_velocity_encoded)
            .map(|_| Edge {
                from: *from,
                to: State {
                    mode: Mode::Skiing { velocity: 0 },
                    ..*from
                },
                cost: 0,
            })
    }

    fn poling_edges(
        &'a self,
        from: &'a State,
    ) -> impl Iterator<Item = ::network::model::Edge<State>> + 'a {
        let max_velocity_encoded = encode_velocity(&POLING_MAX_VELOCITY).unwrap();

        once(from)
            .flat_map(get_skiing_velocity)
            .filter(move |velocity| **velocity <= max_velocity_encoded)
            .flat_map(move |velocity| self.get_poling_edge(from, velocity))
    }

    fn get_poling_edge(&self, from: &State, velocity: &u8) -> Option<Edge<State>> {
        let to_position = self.get_to_position(&from.position, &from.travel_direction)?;

        if (self.is_reserved_fn)(&to_position) {
            return None;
        }

        let initial_velocity: f32 = decode_velocity(velocity)?;

        let run = from.travel_direction.run();
        let rise = self.terrain[to_position] - self.terrain[from.position];
        let physics::skiing::Solution { velocity, duration } =
            physics::skiing::solve(initial_velocity, run, rise, POLING_ACCELERATION, 0.0)?;

        Some(Edge {
            from: *from,
            to: State {
                position: to_position,
                mode: Mode::Skiing {
                    velocity: encode_velocity(&velocity)?,
                },
                travel_direction: from.travel_direction,
            },
            cost: (duration * 1_000_000.0).round() as u32,
        })
    }

    fn skis_off(
        &'a self,
        from: &'a State,
    ) -> impl Iterator<Item = ::network::model::Edge<State>> + 'a {
        once(from)
            .filter(|from| from.mode == Mode::Skiing { velocity: 0 })
            .map(|from| Edge {
                from: *from,
                to: State {
                    mode: Mode::Walking,
                    ..*from
                },
                cost: SKIS_OFF_DURATION.as_micros().try_into().unwrap(),
            })
    }

    fn skis_on(
        &'a self,
        from: &'a State,
    ) -> impl Iterator<Item = ::network::model::Edge<State>> + 'a {
        once(from)
            .filter(|from| from.mode == Mode::Walking)
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

fn get_skiing_velocity(state: &State) -> Option<&u8> {
    match state {
        State {
            mode: Mode::Skiing { velocity },
            ..
        } => Some(velocity),
        _ => None,
    }
}

pub struct SkiingInNetwork {
    pub edges: HashMap<State, Vec<Edge<State>>>,
}

impl SkiingInNetwork {
    pub fn for_states(
        network: &dyn OutNetwork<State>,
        positions: &HashSet<XY<u32>>,
    ) -> SkiingInNetwork {
        let mut edges = HashMap::with_capacity(positions.len());

        for position in positions {
            for travel_direction in DIRECTIONS {
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
                        edges.entry(edge.to).or_insert_with(Vec::new).push(edge);
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
