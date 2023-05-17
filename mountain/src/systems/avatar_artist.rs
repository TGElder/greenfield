use std::collections::hash_map::Entry;
use std::collections::HashMap;

use engine::graphics::Graphics;

use crate::draw::avatar::draw;
use crate::model::Frame;

pub fn run(
    graphics: &mut dyn Graphics,
    frames: &HashMap<usize, Frame>,
    drawings: &mut HashMap<usize, usize>,
) {
    for (id, frame) in frames {
        match drawings.entry(*id) {
            Entry::Occupied(value) => draw(graphics, value.get(), frame),
            Entry::Vacant(cell) => {
                if let Ok(index) = graphics.create_quads() {
                    draw(graphics, &index, frame);
                    cell.insert(index);
                }
            }
        };
    }
}
