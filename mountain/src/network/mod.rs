use std::f32::consts::PI;
use std::iter::once;

use commons::{geometry::XY, grid::Grid, unsafe_float_ordering};
use network::model::{Edge, Network};

use crate::model::Direction;

const GRAVITY: f32 = 9.81;
const MAX_VELOCITY: f32 = 8.0;
const VELOCITY_LEVELS: f32 = 8.0;

pub struct TerrainNetwork<'a> {
    terrain: &'a Grid<f32>,
}

impl<'a> TerrainNetwork<'a> {
    pub fn new(terrain: &'a Grid<f32>) -> TerrainNetwork<'a> {
        TerrainNetwork { terrain }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct State {
    pub position: XY<u32>,
    pub direction: Direction,
    pub velocity: u8,
}

impl<'a> Network<State> for TerrainNetwork<'a> {
    fn edges<'b>(
        &'b self,
        from: &'b State,
    ) -> Box<dyn Iterator<Item = network::model::Edge<State>> + 'b> {
        Box::new(
            [
                from.direction.next_anticlockwise(),
                from.direction,
                from.direction.next_clockwise(),
            ]
            .into_iter()
            .flat_map(|direction| get_next(self.terrain, from, &direction)), // .chain(once(Edge {
                                                                             //     from: *from,
                                                                             //     to: State {
                                                                             //         velocity: 0,
                                                                             //         ..*from
                                                                             //     },
                                                                             //     cost: 0,
                                                                             // })),
        )
    }
}

fn get_next(
    terrain: &Grid<f32>,
    state: &State,
    travel_direction: &Direction,
) -> Option<Edge<State>> {
    let n = terrain.offset(state.position, travel_direction.offset())?;
    let z = terrain[state.position];
    let nz = terrain[n];

    let v = decode_velocity(state.velocity);

    let fall = z - nz;
    let run = travel_direction.run();
    let d = (run.powf(2.0) + fall.powf(2.0)).sqrt();
    let fall = (fall / d).atan() / (PI / 2.0);
    let a = fall * GRAVITY;

    let root = v.powf(2.0) + 2.0 * a * run;
    if root < 0.0 {
        return None;
    }
    let squared = root.sqrt();
    let t = [(-v + squared) / a, (-v - squared) / a]
        .into_iter()
        .filter(|t| *t >= 0.0)
        .min_by(unsafe_float_ordering)?;

    let v_next = v + a * t;
    if !(0.0..=MAX_VELOCITY).contains(&v_next) {
        return None;
    }
    let v_encoded = encode_velocity(v_next);

    let edge = Edge {
        from: *state,
        to: State {
            position: n,
            direction: *travel_direction,
            velocity: v_encoded,
        },
        cost: (t * 1_000_000.0).round() as u32,
    };

    Some(edge)
}

fn encode_velocity(decoded: f32) -> u8 {
    (decoded * (VELOCITY_LEVELS / MAX_VELOCITY)).round() as u8
}

fn decode_velocity(encoded: u8) -> f32 {
    (encoded as f32) * (MAX_VELOCITY / VELOCITY_LEVELS)
}
