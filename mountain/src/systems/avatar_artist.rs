use std::collections::hash_map::Entry;
use std::collections::HashMap;

use engine::graphics::Graphics;

use crate::draw::draw_avatar;
use crate::model::Frame;

pub fn run(
    graphics: &mut dyn Graphics,
    frames: &HashMap<usize, Frame>,
    drawings: &mut HashMap<usize, usize>,
) {
    for (id, frame) in frames {
        match drawings.entry(*id) {
            Entry::Occupied(value) => draw_avatar(value.get(), frame, graphics),
            Entry::Vacant(cell) => {
                if let Ok(index) = graphics.create_quads() {
                    draw_avatar(&index, frame, graphics);
                    cell.insert(index);
                }
            }
        };
    }
}
