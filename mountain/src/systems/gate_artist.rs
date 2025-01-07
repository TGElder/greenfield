use std::collections::hash_map::Entry;
use std::collections::HashMap;

use commons::grid::Grid;
use engine::graphics::{DrawMode, Graphics};

use crate::draw::gate::draw;
use crate::model::entrance::Entrance;
use crate::model::gate::Gate;

pub fn run(
    graphics: &mut dyn Graphics,
    gates: &HashMap<usize, Gate>,
    entrances: &HashMap<usize, Entrance>,
    terrain: &Grid<f32>,
    piste_map: &Grid<Option<usize>>,
    drawings: &mut HashMap<usize, usize>,
) {
    for (gate_id, gate) in gates {
        if let Entry::Vacant(cell) = drawings.entry(*gate_id) {
            if let Ok(index) = graphics.create_triangles(DrawMode::Solid) {
                let Some(entrance) = entrances.get(gate_id) else {
                    continue;
                };
                draw(graphics, &index, gate, entrance, terrain, piste_map);
                cell.insert(index);
            }
        };
    }
}
