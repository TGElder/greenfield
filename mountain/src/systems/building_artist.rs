use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

use commons::grid::Grid;
use engine::graphics::Graphics;

use crate::draw::building::draw;
use crate::model::building::Building;

pub struct System {
    redraw: HashSet<usize>,
}

impl System {
    pub fn new() -> Self {
        Self {
            redraw: HashSet::default(),
        }
    }

    pub fn run(
        &mut self,
        graphics: &mut dyn Graphics,
        buildings: &HashMap<usize, Building>,
        terrain: &Grid<f32>,
        drawings: &mut HashMap<usize, usize>,
    ) {
        for (building_id, building) in buildings {
            match drawings.entry(*building_id) {
                Entry::Vacant(cell) => {
                    if let Ok(index) = graphics.create_triangles() {
                        draw(graphics, &index, building, terrain);
                        cell.insert(index);
                    }
                }
                Entry::Occupied(value) => {
                    let drawing_id = value.get();
                    if self.redraw.contains(building_id) {
                        draw(graphics, drawing_id, building, terrain);
                    }
                }
            };
        }
        self.redraw.clear();
    }

    pub fn redraw(&mut self, building_id: usize) {
        self.redraw.insert(building_id);
    }
}
