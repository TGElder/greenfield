use std::f32::consts::PI;
use std::iter::once;

use commons::{geometry::XY, grid::Grid, unsafe_float_ordering};
use network::model::{Edge, Network};

use crate::model::Direction;

const GRAVITY: f32 = 9.81;
const MAX_VELOCITY: f32 = 8.0;
const VELOCITY_LEVELS: f32 = 8.0;

pub struct TerrainNetwork<'a> {
    pub terrain: &'a Grid<f32>,
}

impl<'a> TerrainNetwork<'a> {
    pub fn new(terrain: &'a Grid<f32>) -> TerrainNetwork<'a> {
        TerrainNetwork { terrain }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct State {
    pub position: XY<u32>,
    pub travel_direction: Direction,
    pub body_direction: Direction,
    pub velocity: u8,
}

impl<'a> Network<State> for TerrainNetwork<'a> {
    fn edges<'b>(
        &'b self,
        from: &'b State,
    ) -> Box<dyn Iterator<Item = network::model::Edge<State>> + 'b> {
        Box::new(
            [
                from.travel_direction.next_anticlockwise(),
                from.travel_direction,
                from.travel_direction.next_clockwise(),
            ]
            .into_iter()
            .flat_map(|travel_direction| {
                [
                    from.body_direction.next_anticlockwise(),
                    from.body_direction,
                    from.body_direction.next_clockwise(),
                ]
                .into_iter()
                .map(move |body_direction| (travel_direction, body_direction))
            })
            .filter(|(travel_direction, body_direction)| {
                *travel_direction == body_direction.next_anticlockwise()
                    || *travel_direction == *body_direction
                    || *travel_direction == body_direction.next_clockwise()
            })
            .flat_map(|(travel_direction, body_direction)| {
                get_next(self.terrain, from, &travel_direction, &body_direction)
            }), // .chain(once(Edge {
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
    body_direction: &Direction,
) -> Option<Edge<State>> {
    let n = terrain.offset(state.position, travel_direction.offset())?;

    let (t, v, mut a) = get_t_v_a(state.position, n, state.velocity, terrain)?;

    if body_direction != travel_direction {
        a -= (v * v) / 128.0;
    }

    let v_next = v + a * t;
    if !(0.0..=MAX_VELOCITY).contains(&v_next) {
        return None;
    }
    let v_encoded = encode_velocity(v_next);

    let edge = Edge {
        from: *state,
        to: State {
            position: n,
            travel_direction: *travel_direction,
            body_direction: *body_direction,
            velocity: v_encoded,
        },
        cost: (t * 1_000_000.0).round() as u32,
    };

    Some(edge)
}

pub fn get_t_v_a(
    from: XY<u32>,
    to: XY<u32>,
    velocity: u8,
    terrain: &Grid<f32>,
) -> Option<(f32, f32, f32)> {
    let z = terrain[from];
    let nz = terrain[to];

    let v = decode_velocity(velocity);

    let fall = z - nz;
    let run_squared = (from.x - to.x).pow(2) as f32 + (from.y - to.y).pow(2) as f32;
    let run = run_squared.sqrt();
    // let d = (run_squared + fall.powf(2.0)).sqrt();
    let fall = (fall / run).atan() / (PI / 2.0);
    let a = fall * GRAVITY;
    // let drag = (v * v) / 128.0;
    // let a = a - drag;

    let root = v.powf(2.0) + 2.0 * a * (run_squared + fall.powf(2.0)).sqrt();
    if root < 0.0 {
        return None;
    }
    let squared = root.sqrt();
    [(-v + squared) / a, (-v - squared) / a]
        .into_iter()
        .filter(|t| *t >= 0.0)
        .min_by(unsafe_float_ordering)
        .map(|t| (t, v, a))
}

fn encode_velocity(decoded: f32) -> u8 {
    (decoded * (VELOCITY_LEVELS / MAX_VELOCITY)).round() as u8
}

fn decode_velocity(encoded: u8) -> f32 {
    (encoded as f32) * (MAX_VELOCITY / VELOCITY_LEVELS)
}

pub fn min_time(encoded: u8) -> u64 {
    ((1.0f32 / decode_velocity(encoded)) * 1_000_000.0).round() as u64
}
