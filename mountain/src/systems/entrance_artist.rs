use std::collections::hash_map::Entry;
use std::collections::HashMap;

use commons::grid::Grid;
use engine::graphics::Graphics;

use crate::draw::entrance::draw;
use crate::model::entrance::Entrance;

pub fn run(
    graphics: &mut dyn Graphics,
    entrances: &HashMap<usize, Entrance>,
    terrain: &Grid<f32>,
    piste_map: &Grid<Option<usize>>,
    drawings: &mut HashMap<usize, usize>,
) {
    for (id, entrance) in entrances {
        if let Entry::Vacant(cell) = drawings.entry(*id) {
            if let Ok(index) = graphics.create_quads() {
                draw(graphics, &index, entrance, terrain, piste_map);
                cell.insert(index);
            }
        };
    }
}
