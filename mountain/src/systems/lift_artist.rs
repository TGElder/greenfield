use std::collections::hash_map::Entry;
use std::collections::HashMap;

use engine::graphics::Graphics;

use crate::draw::line::draw;
use crate::model::lift::{Lift, Segment};

pub fn run(
    graphics: &mut dyn Graphics,
    lifts: &HashMap<usize, Lift>,
    drawings: &mut HashMap<usize, usize>,
) {
    for (id, lift) in lifts {
        let segments = lift
            .segments
            .iter()
            .map(|Segment { from, to, .. }| [from, to])
            .collect::<Vec<_>>();
        match drawings.entry(*id) {
            Entry::Occupied(value) => draw(graphics, value.get(), &segments, 0.5),
            Entry::Vacant(cell) => {
                if let Ok(index) = graphics.create_quads() {
                    draw(graphics, &index, &segments, 0.5);
                    cell.insert(index);
                }
            }
        };
    }
}
