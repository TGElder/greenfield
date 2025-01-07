use std::collections::hash_map::Entry;
use std::collections::HashMap;

use commons::grid::Grid;
use engine::graphics::{DrawMode, Graphics};

use crate::draw::door::draw;
use crate::model::building::Building;
use crate::model::door::Door;

pub fn run(
    graphics: &mut dyn Graphics,
    doors: &HashMap<usize, Door>,
    buildings: &HashMap<usize, Building>,
    terrain: &Grid<f32>,
    drawings: &mut HashMap<usize, usize>,
) {
    for (door_id, door) in doors {
        let Some(building) = buildings.get(&door.building_id) else {
            continue;
        };
        if let Entry::Vacant(cell) = drawings.entry(*door_id) {
            if let Ok(index) = graphics.create_triangles(DrawMode::Solid) {
                draw(graphics, &index, door, building, terrain);
                cell.insert(index);
            }
        };
    }
}
