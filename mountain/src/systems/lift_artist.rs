use std::collections::hash_map::Entry;
use std::collections::HashMap;

use engine::graphics::Graphics;

use crate::draw::line::draw;
use crate::model::lift::Lift;

pub fn run(
    graphics: &mut dyn Graphics,
    lifts: &HashMap<usize, Lift>,
    drawings: &mut HashMap<usize, usize>,
) {
    for (id, lift) in lifts {
        let from = lift.nodes[0].from;
        let to = lift.nodes[1].to;
        match drawings.entry(*id) {
            Entry::Occupied(value) => draw(graphics, value.get(), &from, &to, 0.5),
            Entry::Vacant(cell) => {
                if let Ok(index) = graphics.create_quads() {
                    draw(graphics, &index, &from, &to, 0.5);
                    cell.insert(index);
                }
            }
        };
    }
}
