use std::collections::hash_map::Entry;
use std::collections::HashMap;

use commons::grid::Grid;
use engine::graphics::Graphics;

use crate::draw::structure::draw_chain;
use crate::handlers::structure_builder;
use crate::model::structure::Structure;
use crate::services::id_allocator;

pub struct System {
    index: Option<usize>,
    wire_index: Option<usize>,
}

impl System {
    pub fn new() -> System {
        System {
            index: None,
            wire_index: None,
        }
    }

    pub fn run(
        &mut self,
        graphics: &mut dyn Graphics,
        structures: &HashMap<usize, Structure>,
        terrain: &Grid<f32>,
        structure_builder: &structure_builder::Handler,
        id_allocator: &mut id_allocator::Service,
        drawings: &mut HashMap<usize, usize>,
    ) {
        let index = self.index.get_or_insert_with(|| id_allocator.next_id());
        let wire_index = self
            .wire_index
            .get_or_insert_with(|| id_allocator.next_id());

        if let Entry::Vacant(cell) = drawings.entry(*index) {
            if let Ok(index) = graphics.create_hologram() {
                cell.insert(index);
            }
        };

        if let Entry::Vacant(cell) = drawings.entry(*wire_index) {
            if let Ok(index) = graphics.create_hologram() {
                cell.insert(index);
            }
        };

        let structures = structure_builder
            .structures
            .iter()
            .flat_map(|structure| structures.get(structure))
            .collect::<Vec<_>>();

        draw_chain(
            graphics,
            &drawings[index],
            &drawings[wire_index],
            &structures,
            terrain,
        );
    }
}
