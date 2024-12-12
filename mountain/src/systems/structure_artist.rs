use std::collections::hash_map::Entry;
use std::collections::HashMap;

use commons::grid::Grid;
use engine::graphics::Graphics;

use crate::draw::structure::draw;
use crate::model::structure::Structure;

pub fn run(
    graphics: &mut dyn Graphics,
    structures: &HashMap<usize, Structure>,
    terrain: &Grid<f32>,
    drawings: &mut HashMap<usize, usize>,
) {
    for (structure_id, structure) in structures {
        if let Entry::Vacant(cell) = drawings.entry(*structure_id) {
            if let Ok(index) = graphics.create_triangles() {
                draw(graphics, &index, structure, terrain);
                cell.insert(index);
            }
        };
    }
}
