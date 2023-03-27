use commons::{geometry::XY, grid::Grid, unsafe_float_ordering};

use crate::model::Direction;

const GRAVITY: f32 = 9.81;

struct State {
    position: XY<u32>,
    direction: Direction,
    speed: f32,
}

fn edges<'a>(terrain: &'a Grid<f32>, state: &'a State) -> impl Iterator<Item = State> + 'a {
    [
        state.direction.next_anticlockwise(),
        state.direction,
        state.direction.next_clockwise(),
    ]
    .into_iter()
    .flat_map(|direction| get_next(terrain, state, &direction))
}

fn get_next(terrain: &Grid<f32>, state: &State, travel_direction: &Direction) -> Option<State> {
    let n = terrain.offset(&state.position, travel_direction.offset())?;
    let z = terrain[state.position];
    let nz = terrain[n];

    let v = state.speed;

    let fall = nz - z;
    let run = travel_direction.run();
    let d = (run.powf(2.0) + fall.powf(2.0)).sqrt();
    let fall = fall / d;
    let a = fall * GRAVITY;

    let root = v.powf(2.0) - 2.0 * a * run;
    if root < 0.0 {
        return None;
    }
    let squared = root.sqrt();
    let t = [(-v + squared) / a, (-v - squared) / a]
        .into_iter()
        .filter(|t| *t >= 0.0)
        .min_by(unsafe_float_ordering)?;

    let v_next = v + a * t;
    if v_next < 0.0 || v_next > 22.352 {
        return None;
    }

    Some(State {
        position: n,
        direction: *travel_direction,
        speed: v_next,
    })
}
