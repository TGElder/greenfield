use std::f32::consts::PI;

use commons::geometry::XYZ;
use commons::scale::Scale;

use crate::model::Direction;

#[derive(Debug)]
pub enum Avatar {
    _Static(State),
    Moving(Vec<Frame>),
}

#[derive(Clone, Copy, Debug)]
pub struct State {
    pub position: XYZ<f32>,
    pub angle: f32,
}

#[derive(Debug)]
pub struct Frame {
    pub arrival_micros: u64,
    pub state: State,
}

impl Avatar {
    pub fn state(&self, micros: &u64) -> Option<State> {
        match self {
            Avatar::_Static(state) => Some(*state),
            Avatar::Moving(frames) => state(frames, micros),
        }
    }
}

fn state(frames: &[Frame], micros: &u64) -> Option<State> {
    let maybe_pair = frames.windows(2).find(|maybe_pair| match maybe_pair {
        [from, to] => from.arrival_micros <= *micros && to.arrival_micros > *micros,
        _ => false,
    });
    match maybe_pair {
        Some([from, to]) => Some(blend(from, to, micros)),
        _ => None,
    }
}

fn blend(from: &Frame, to: &Frame, micros: &u64) -> State {
    let scale = Scale::new(
        (from.arrival_micros as f32, to.arrival_micros as f32),
        (0.0, 1.0),
    );
    let p = scale.scale(*micros as f32);
    
    if (from.state.angle - to.state.angle).abs() > PI { // rotating opposite direction is quicker
        if from.state.angle < to.state.angle {
            State {
                position: from.state.position * (1.0 - p) + to.state.position * p,
                angle: (from.state.angle + 2.0 * PI)  * (1.0 - p) + to.state.angle * p,
            }
        } else {
            State {
                position: from.state.position * (1.0 - p) + to.state.position * p,
                angle: from.state.angle * (1.0 - p) + (to.state.angle + 2.0 * PI) * p,
            }
        }
    } else {

        State {
            position: from.state.position * (1.0 - p) + to.state.position * p,
            angle: from.state.angle * (1.0 - p) + to.state.angle * p,
        }
    }
}
