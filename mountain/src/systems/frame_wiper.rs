use std::collections::HashMap;

use crate::model::frame::Frame;

pub fn run(frames: &mut HashMap<usize, Option<Frame>>) {
    frames.iter_mut().for_each(|(_, frame)| *frame = None);
}
