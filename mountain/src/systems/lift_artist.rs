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
        let points = lift
            .segments
            .iter()
            .map(|segment| segment.from)
            .collect::<Vec<_>>();
        match drawings.entry(*id) {
            Entry::Occupied(value) => draw(graphics, value.get(), &points, 0.5),
            Entry::Vacant(cell) => {
                if let Ok(index) = graphics.create_quads() {
                    draw(graphics, &index, &points, 0.5);
                    cell.insert(index);
                }
            }
        };
    }
}
