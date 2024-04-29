use std::collections::hash_map::Entry;
use std::collections::HashMap;

use commons::grid::Grid;
use engine::graphics::Graphics;

use crate::draw::gate::draw;
use crate::model::gate::Gate;

pub fn run(
    graphics: &mut dyn Graphics,
    gates: &HashMap<usize, Gate>,
    terrain: &Grid<f32>,
    piste_map: &Grid<Option<usize>>,
    drawings: &mut HashMap<usize, usize>,
) {
    for (id, gate) in gates {
        if let Entry::Vacant(cell) = drawings.entry(*id) {
            if let Ok(index) = graphics.create_triangles() {
                draw(graphics, &index, gate, terrain, piste_map);
                cell.insert(index);
            }
        };
    }
}
