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
        let from = xyz(
            lift.pick_up.position.x as f32,
            lift.pick_up.position.y as f32,
            terrain[lift.pick_up.position],
        );
        let to = xyz(
            lift.drop_off.position.x as f32,
            lift.drop_off.position.y as f32,
            terrain[lift.drop_off.position],
        );
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
