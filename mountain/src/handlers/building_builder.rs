use std::collections::HashMap;

use commons::geometry::{xy, XYRectangle};
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

        let rectangle = XYRectangle {
            from: *grid.origin(),
            to: *grid.origin() + xy(grid.width() + 1, grid.height() + 1),
        };

        let id = id_allocator.next_id();
        let building = Building {
            footprint: rectangle,
            height: 6,
        };

        buildings.insert(id, building);

        selection.clear_selection();
    }
}
