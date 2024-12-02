use std::collections::{HashMap, HashSet};
use std::iter::{empty, once};
use std::time::Duration;

use commons::{geometry::XY, grid::Grid};
use network::model::{Edge, InNetwork, OutNetwork};

use crate::model::ability::Ability;
use crate::model::direction::{Direction, DIRECTIONS};
use crate::model::piste;
use crate::model::skiing::State;
use crate::utils::ability::exposure;
use crate::{
    network::velocity_encoding::{decode_velocity, encode_velocity},
    utils::physics,
};

const TURNING_DURATION: Duration = Duration::from_secs(1);
const WALK_DURATION: Duration = Duration::from_micros(1_000_000);
const WALK_DIAGONAL_DURATION: Duration = Duration::from_micros(1_414_214);

const BRAKING_FRICTION: f32 = 1.0;

const POLING_ACCELERATION: f32 = 1.0;
const POLING_MAX_VELOCITY: f32 = 1.0;

pub struct SkiingNetwork<'a> {
    pub terrain: &'a Grid<f32>,
    pub class: piste::Class,
    pub ability: Ability,
    pub is_accessible_fn: &'a dyn Fn(&XY<u32>) -> bool,
    pub is_valid_edge_fn: &'a dyn Fn(&State, &State) -> bool,
}

impl OutNetwork<State> for SkiingNetwork<'_> {
    fn edges_out<'b>(
        &'b self,
        from: &'b State,
    ) -> Box<dyn Iterator<Item = ::network::model::Edge<State>> + 'b> {
        match self.class {
            piste::Class::Piste => Box::new(self.piste_edges(from)),
            piste::Class::Path => Box::new(self.path_edges(from)),
        }
    }
}

impl<'a> SkiingNetwork<'a> {
    fn piste_edges<'b>(
        &'b self,
        from: &'b State,
    ) -> impl Iterator<Item = ::network::model::Edge<State>> + 'b {
        self.poling_edges(from)
            .chain(self.skiing_edges(from))
            .chain(self.braking_edges(from))
            .filter(|edge| exposure(self.terrain, &edge.to.position) <= self.ability.max_exposure())
            .chain(self.turning_edges(from))
            .filter(|edge| (self.is_valid_edge_fn)(&edge.from, &edge.to))
            .chain(self.stop_edge(from))
    }

    fn path_edges<'b>(
        &'b self,
        from: &'b State,
    ) -> impl Iterator<Item = ::network::model::Edge<State>> + 'b {
        self.walking_edges(from)
            .chain(self.turning_edges(from))
            .chain(self.stop_edge(from))
    }

    fn skiing_edges(
        &'a self,
        from: &'a State,
    ) -> impl Iterator<Item = ::network::model::Edge<State>> + 'a {
        once(from).flat_map(|state| {
            [
                from.travel_direction.next_anticlockwise(),
                from.travel_direction,
                from.travel_direction.next_clockwise(),
            ]
            .into_iter()
            .flat_map(|travel_direction| {
                self.get_skiing_edge(from, travel_direction, state.velocity, 0.0)
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

        if !(self.is_accessible_fn)(&to_position) {
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

                velocity: encode_velocity(&velocity)?,
                travel_direction,
            },
            cost: (duration * 1_000_000.0).round() as u32,
        })
    }

    fn braking_edges(
        &'a self,
        from: &'a State,
    ) -> impl Iterator<Item = ::network::model::Edge<State>> + 'a {
        once(from).flat_map(move |state| {
            self.get_skiing_edge(
                from,
                from.travel_direction,
                state.velocity,
                BRAKING_FRICTION,
            )
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
            .filter(|from| from.velocity == 0)
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
        once(Edge {
            from: *from,
            to: from.stationary(),
            cost: 0,
        })
    }

    fn poling_edges(
        &'a self,
        from: &'a State,
    ) -> impl Iterator<Item = ::network::model::Edge<State>> + 'a {
        let max_velocity_encoded = encode_velocity(&POLING_MAX_VELOCITY).unwrap();

        once(from)
            .filter(move |state| state.velocity <= max_velocity_encoded)
            .flat_map(|state| self.get_poling_edge(from, &state.velocity))
    }

    fn get_poling_edge(&self, from: &State, velocity: &u8) -> Option<Edge<State>> {
        let to_position = self.get_to_position(&from.position, &from.travel_direction)?;

        if !(self.is_accessible_fn)(&to_position) {
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
                velocity: encode_velocity(&velocity)?,
                travel_direction: from.travel_direction,
            },
            cost: (duration * 1_000_000.0).round() as u32,
        })
    }

    fn get_to_position(&self, position: &XY<u32>, travel_direction: &Direction) -> Option<XY<u32>> {
        let offset = travel_direction.offset();
        self.terrain.offset(position, offset)
    }

    fn walking_edges<'b>(
        &'b self,
        from: &'b State,
    ) -> impl Iterator<Item = ::network::model::Edge<State>> + 'b {
        once(from)
            .filter(|from| from.velocity == 0)
            .flat_map(|from| {
                DIRECTIONS
                    .iter()
                    .flat_map(|direction| {
                        self.terrain
                            .offset(from.position, direction.offset())
                            .map(|neighbour| (direction, neighbour))
                    })
                    .filter(|(_, neighbour)| (self.is_accessible_fn)(neighbour))
                    .map(|(&direction, neighbour)| Edge {
                        from: *from,
                        to: State {
                            position: neighbour,
                            travel_direction: direction,
                            velocity: 0,
                        },
                        cost: walk_duration(direction).as_micros().try_into().unwrap(),
                    })
            })
    }
}

fn walk_duration(direction: Direction) -> Duration {
    match direction {
        Direction::North | Direction::East | Direction::South | Direction::West => WALK_DURATION,
        Direction::NorthEast
        | Direction::SouthEast
        | Direction::SouthWest
        | Direction::NorthWest => WALK_DIAGONAL_DURATION,
    }
}

pub struct StationaryNetwork {
    pub edges: HashMap<State, Vec<Edge<State>>>,
}

impl StationaryNetwork {
    pub fn for_positions(
        network: &dyn OutNetwork<State>,
        positions: &HashSet<XY<u32>>,
    ) -> StationaryNetwork {
        let mut edges = HashMap::with_capacity(positions.len());

        for position in positions {
            for travel_direction in DIRECTIONS {
                let state = State {
                    position: *position,
                    velocity: 0,
                    travel_direction,
                };

                for edge in network
                    .edges_out(&state)
                    .filter(|Edge { to, .. }| positions.contains(&to.position))
                {
                    let edge = Edge {
                        to: edge.to.stationary(),
                        ..edge
                    };
                    edges.entry(edge.to).or_insert_with(Vec::new).push(edge);
                }
            }
        }

        StationaryNetwork { edges }
    }
}

impl InNetwork<State> for StationaryNetwork {
    fn edges_in<'a>(&'a self, to: &'a State) -> Box<dyn Iterator<Item = Edge<State>> + 'a> {
        match self.edges.get(to) {
            Some(edges) => Box::new(edges.iter().copied()),
            None => Box::new(empty()),
        }
    }
}
