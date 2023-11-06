use std::collections::hash_map::Entry;
use std::collections::HashMap;

use engine::graphics::Graphics;

use crate::draw::model::draw;
use crate::model::frame::Frame;

pub fn run(
    graphics: &mut dyn Graphics,
    frames: &HashMap<usize, Option<Frame>>,
    drawings: &mut HashMap<usize, usize>,
) {
    for (id, frame) in frames {
        let entry = drawings.entry(*id);
        let index = match entry {
            Entry::Occupied(ref value) => value.get(),
            Entry::Vacant(cell) => {
                let Ok(index) = graphics.create_quads() else {
                    continue;
                };
                &*cell.insert(index)
            }
        };

        match frame {
            Some(frame) => draw(graphics, index, frame),
            None => graphics.draw_quads(index, &[]).unwrap(),
        }
    }
}
