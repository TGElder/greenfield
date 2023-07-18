use std::iter::once;
use std::time::Duration;

use commons::{geometry::XY, grid::Grid};
use network::model::{Edge, OutNetwork};

use crate::model::direction::Direction;
use crate::model::skiing::{Mode, State};
use crate::{
    network::velocity_encoding::{decode_velocity, encode_velocity},
    physics,
};

const TURNING_DURATION: Duration = Duration::from_secs(1);
const SKIS_ON_DURATION: Duration = Duration::from_secs(10);
const SKIS_OFF_DURATION: Duration = Duration::from_secs(10);
const WALK_DURATION: Duration = Duration::from_secs(1);

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
                .chain(self.skis_off(from))
                .chain(self.skis_on(from))
                .chain(self.walk(from)),
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
        once(from)
            .flat_map(get_skiing_velocity)
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

    fn walk(&'a self, from: &'a State) -> impl Iterator<Item = ::network::model::Edge<State>> + 'a {
        once(from)
            .filter(|from| from.mode == Mode::Walking)
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
