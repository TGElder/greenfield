use std::collections::HashMap;

use commons::geometry::xyz;
use commons::grid::Grid;
use commons::scale::Scale;

use crate::model::Behavior;
use crate::model::{Event, Frame};
use crate::network::skiing;

pub fn run(
    terrain: &Grid<f32>,
    micros: &u128,
    behaviors: &HashMap<usize, Behavior>,
    frames: &mut HashMap<usize, Frame>,
) {
    for (i, behavior) in behaviors {
        let frame = frame_from_behavior(terrain, behavior, micros);
        match frame {
            Some(frame) => frames.insert(*i, frame),
            None => frames.remove(i),
        };
    }
}

fn frame_from_behavior(terrain: &Grid<f32>, behavior: &Behavior, micros: &u128) -> Option<Frame> {
    match behavior {
        Behavior::_Stationary(state) => Some(frame_from_skiing_state(terrain, state)),
        Behavior::Moving(events) => moving_frame(terrain, events, micros),
    }
}

fn moving_frame(terrain: &Grid<f32>, events: &[Event], micros: &u128) -> Option<Frame> {
    let maybe_pair = events.windows(2).find(|maybe_pair| match maybe_pair {
        [from, to] => from.micros as u128 <= *micros && to.micros as u128 > *micros,
        _ => false,
    });
    match maybe_pair {
        Some([from, to]) => Some(blend(terrain, from, to, micros)),
        _ => None,
    }
}

fn blend(terrain: &Grid<f32>, from: &Event, to: &Event, micros: &u128) -> Frame {
    let scale = Scale::new((from.micros as f32, to.micros as f32), (0.0, 1.0));
    let p = scale.scale(*micros as f32);
    let from = frame_from_skiing_state(terrain, &from.state);
    let to = frame_from_skiing_state(terrain, &to.state);
    Frame {
        position: from.position * (1.0 - p) + to.position * p,
        angle: from.angle,
    }
}

fn frame_from_skiing_state(
    terrain: &Grid<f32>,
    skiing::State {
        position,
        travel_direction,
        ..
    }: &skiing::State,
) -> Frame {
    Frame {
        position: xyz(position.x as f32, position.y as f32, terrain[position]),
        angle: travel_direction.angle(),
    }
}
