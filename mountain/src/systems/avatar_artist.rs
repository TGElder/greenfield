use std::collections::hash_map::Entry;
use std::collections::HashMap;

use engine::graphics::Graphics;

use crate::draw::avatar::draw;
use crate::model::frame::Frame;

pub fn run(
    graphics: &mut dyn Graphics,
    frames: &HashMap<usize, Option<Frame>>,
    drawings: &mut HashMap<usize, usize>,
) {
    for (id, frame) in frames {
        match drawings.entry(*id) {
            Entry::Occupied(value) => match frame {
                Some(frame) => draw(graphics, value.get(), frame),
                None => graphics.draw_quads(value.get(), &[]).unwrap(),
            },
            Entry::Vacant(cell) => {
                if let Ok(index) = graphics.create_quads() {
                    match frame {
                        Some(frame) => draw(graphics, &index, frame),
                        None => graphics.draw_quads(&index, &[]).unwrap(),
                    }
                    cell.insert(index);
                }
            }
        };
    }
}
