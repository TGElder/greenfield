use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

use commons::grid::Grid;
use engine::graphics::Graphics;

use crate::draw::lift_building::draw;
use crate::model::lift_building::LiftBuildings;

#[derive(Default)]
pub struct System {
    to_draw: HashSet<usize>,
}

impl System {
    pub fn redraw(&mut self, id: usize) {
        self.to_draw.insert(id);
    }

    pub fn run(
        &mut self,
        graphics: &mut dyn Graphics,
        lift_buildings: &HashMap<usize, LiftBuildings>,
        terrain: &Grid<f32>,
        drawings: &mut HashMap<usize, usize>,
    ) {
        for id in self.to_draw.drain() {
            let Some(lift_buildings) = lift_buildings.get(&id) else {
                continue;
            };
            let Some(graphics_index) = try_get_graphics_index(&id, drawings, graphics) else {
                continue;
            };
            draw(graphics, &graphics_index, lift_buildings, terrain);
        }
    }
}

fn try_get_graphics_index(
    index: &usize,
    drawings: &mut HashMap<usize, usize>,
    graphics: &mut dyn Graphics,
) -> Option<usize> {
    match drawings.entry(*index) {
        Entry::Occupied(cell) => Some(*cell.get()),
        Entry::Vacant(cell) => {
            if let Ok(graphics_index) = graphics.create_triangles() {
                cell.insert(graphics_index);
                Some(graphics_index)
            } else {
                None
            }
        }
    }
}
