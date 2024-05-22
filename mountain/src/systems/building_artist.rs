use std::collections::hash_map::Entry;
use std::collections::HashMap;

use commons::grid::Grid;
use engine::graphics::Graphics;

use crate::draw::building::draw;
use crate::model::building::Building;

pub fn run(
    graphics: &mut dyn Graphics,
    buildings: &HashMap<usize, Building>,
    terrain: &Grid<f32>,
    drawings: &mut HashMap<usize, usize>,
) {
    for (building_id, building) in buildings {
        if let Entry::Vacant(cell) = drawings.entry(*building_id) {
            if let Ok(index) = graphics.create_triangles() {
                draw(graphics, &index, building, terrain);
                cell.insert(index);
            }
        };
    }
}
