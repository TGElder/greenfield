use std::collections::HashMap;

use engine::binding::Binding;

use crate::handlers::selection;
use crate::model::building::Building;
use crate::services::id_allocator;

pub struct Handler {
    pub binding: Binding,
}

pub struct Parameters<'a> {
    pub event: &'a engine::events::Event,
    pub selection: &'a mut selection::Handler,
    pub id_allocator: &'a mut id_allocator::Service,
    pub buildings: &'a mut HashMap<usize, Building>,
}

impl Handler {
    pub fn handle(
        &mut self,
        Parameters {
            event,
            selection,
            id_allocator,
            buildings,
        }: Parameters<'_>,
    ) {
        if !self.binding.binds_event(event) {
            return;
        }

        let Some(grid) = &selection.grid else {
            return;
        };

        for position in grid.iter() {
            if !grid[position] {
                // can only build rectangle buildings
                return;
            }
        }

        let Ok(rectangle) = grid.rectangle() else {
            return;
        };

        let id = id_allocator.next_id();
        let building = Building {
            footprint: rectangle,
            height: 1,
        };

        buildings.insert(id, building);
    }
}
