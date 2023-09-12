use std::collections::hash_map::Entry;
use std::collections::HashMap;

use commons::geometry::xyz;
use commons::grid::Grid;
use engine::graphics::Graphics;

use crate::draw::line::draw;
use crate::model::lift::Lift;

pub fn run(
    graphics: &mut dyn Graphics,
    lifts: &HashMap<usize, Lift>,
    terrain: &Grid<f32>,
    drawings: &mut HashMap<usize, usize>,
) {
    for (id, lift) in lifts {
        let from = xyz(lift.from.x as f32, lift.from.y as f32, terrain[lift.from]);
        let to = xyz(lift.to.x as f32, lift.to.y as f32, terrain[lift.to]);
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
