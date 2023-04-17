use commons::geometry::XYZ;
use commons::scale::Scale;

pub struct Animation {
    pub index: usize,
    pub frames: Vec<Frame>,
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

impl Animation {
    pub fn state(&self, micros: &u64) -> Option<State> {
        let frames = &self.frames;
        let maybe_pair = frames.windows(2).find(|maybe_pair| match maybe_pair {
            [from, to] => from.arrival_micros <= *micros && to.arrival_micros > *micros,
            _ => false,
        });
        match maybe_pair {
            Some([from, to]) => Some(blend(from, to, micros)),
            _ => None,
        }
    }
}

fn blend(from: &Frame, to: &Frame, micros: &u64) -> State {
    let scale = Scale::new(
        (from.arrival_micros as f32, to.arrival_micros as f32),
        (0.0, 1.0),
    );
    let p = scale.scale(*micros as f32);
    State {
        position: from.state.position * (1.0 - p) + to.state.position * p,
        angle: from.state.angle,
    }
}
