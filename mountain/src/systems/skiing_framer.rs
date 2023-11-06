use std::collections::HashMap;

use commons::geometry::xyz;
use commons::grid::Grid;
use commons::scale::Scale;

use crate::model::frame::{self, Frame};
use crate::model::skiing::{self, Event, Plan, State};

pub fn run(
    terrain: &Grid<f32>,
    micros: &u128,
    plans: &HashMap<usize, Plan>,
    frames: &mut HashMap<usize, Option<Frame>>,
) {
    for (id, plan) in plans {
        if let Some(frame) = frame_from_plan(terrain, micros, plan) {
            frames.insert(*id, Some(frame));
        };
    }
}

fn frame_from_plan(terrain: &Grid<f32>, micros: &u128, plan: &Plan) -> Option<Frame> {
    match plan {
        Plan::Stationary(state) => Some(frame_from_skiing_state(terrain, state)),
        Plan::Moving(events) => moving_frame(terrain, events, micros),
    }
}

fn frame_from_skiing_state(
    terrain: &Grid<f32>,
    State {
        position,
        mode,
        travel_direction,
    }: &State,
) -> Frame {
    Frame {
        position: xyz(position.x as f32, position.y as f32, terrain[position]),
        angle: travel_direction.angle(),
        model_offset: None,
        model: mode.into(),
    }
}

fn moving_frame(terrain: &Grid<f32>, events: &[Event], micros: &u128) -> Option<Frame> {
    let maybe_pair = events.windows(2).find(|maybe_pair| match maybe_pair {
        [from, to] => from.micros <= *micros && to.micros > *micros,
        _ => false,
    });
    match maybe_pair {
        Some([from, to]) => Some(blend(terrain, micros, from, to)),
        _ => None,
    }
}

fn blend(terrain: &Grid<f32>, micros: &u128, from: &Event, to: &Event) -> Frame {
    let scale = Scale::new((from.micros as f32, to.micros as f32), (0.0, 1.0));
    let p = scale.scale(*micros as f32);
    let from = frame_from_skiing_state(terrain, &from.state);
    let to = frame_from_skiing_state(terrain, &to.state);
    Frame {
        position: from.position * (1.0 - p) + to.position * p,
        angle: to.angle,
        model: from.model,
        model_offset: None,
    }
}

impl From<&skiing::Mode> for frame::Model {
    fn from(value: &skiing::Mode) -> Self {
        match value {
            skiing::Mode::Walking => frame::Model::Standing { skis: false },
            skiing::Mode::Skiing { .. } => frame::Model::Standing { skis: true },
        }
    }
}
