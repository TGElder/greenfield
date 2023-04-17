use std::collections::HashMap;

use engine::graphics::Graphics;

use crate::draw::draw_avatar;
use crate::model::Frame;

pub fn run(
    graphics: &mut dyn Graphics,
    frames: &HashMap<usize, Frame>,
    drawings: &mut HashMap<usize, usize>,
) {
    for (i, frame) in frames {
        // TODO avoid dereference
        let index = drawings
            .entry(*i)
            .or_insert_with(|| graphics.create_quads().unwrap()); // TODO handle error
        draw_avatar(index, frame, graphics);
    }
}
