use std::collections::hash_map::Entry;
use std::collections::HashMap;

use commons::grid::Grid;
use engine::graphics::Graphics;

use crate::draw::door::draw;
use crate::model::door::Door;

pub fn run(
    graphics: &mut dyn Graphics,
    door: &HashMap<usize, Door>,
    terrain: &Grid<f32>,
    drawings: &mut HashMap<usize, usize>,
) {
    for (door_id, door) in door {
        if let Entry::Vacant(cell) = drawings.entry(*door_id) {
            if let Ok(index) = graphics.create_triangles() {
                draw(graphics, &index, door, terrain);
                cell.insert(index);
            }
        };
    }
}
