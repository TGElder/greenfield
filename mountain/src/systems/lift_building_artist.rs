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
        for (lift_building_id, lift_buildings) in lift_buildings {
            if let Entry::Vacant(cell) = drawings.entry(*lift_building_id) {
                if let Ok(graphics_index) = graphics.create_triangles() {
                    cell.insert(graphics_index);
                    draw(graphics, &graphics_index, lift_buildings, terrain);
                }
            }
        }

        for lift_building_id in self.to_draw.drain() {
            let Some(lift_buildings) = lift_buildings.get(&lift_building_id) else {
                continue;
            };
            let Some(drawing_id) = drawings.get(&lift_building_id) else {
                continue;
            };
            draw(graphics, drawing_id, lift_buildings, terrain);
        }
    }
}
