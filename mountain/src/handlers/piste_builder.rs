use std::collections::hash_map::Entry;
use std::collections::HashMap;

use commons::geometry::{xy, XYRectangle};
use commons::grid::Grid;
use commons::origin_grid::OriginGrid;
use engine::binding::Binding;

use crate::handlers::selection;
use crate::model::piste::Piste;
use crate::services::id_allocator;
use crate::systems::overlay;

pub struct Handler {
    pub id: usize,
    pub bindings: Bindings,
}

pub struct Bindings {
    pub add: Binding,
    pub subtract: Binding,
    pub new: Binding,
}

impl Handler {
    pub fn handle(
        &mut self,
        event: &engine::events::Event,
        pistes: &mut HashMap<usize, Piste>,
        piste_map: &mut Grid<Option<usize>>,
        selection: &mut selection::Handler,
        overlay: &mut overlay::System,
        id_allocator: &mut id_allocator::Service,
    ) {
        let add = self.bindings.add.binds_event(event) || self.bindings.new.binds_event(event);
        let subtract = self.bindings.subtract.binds_event(event);
        if !(add || subtract) {
            return;
        }

        let Some(rectangle) = selection.selected_rectangle() else {
            return;
        };
        let mut grid = OriginGrid::from_rectangle(
            XYRectangle {
                from: rectangle.from,
                to: xy(rectangle.to.x + 1, rectangle.to.y + 1),
            },
            false,
        );

        if pistes.is_empty() || self.bindings.new.binds_event(event) {
            self.id = id_allocator.next_id();
        }

        if add {
            for position in grid.iter() {
                if piste_map[position].is_none() {
                    grid[position] = true;
                    piste_map[position] = Some(self.id)
                } else if piste_map[position] == Some(self.id) {
                    grid[position] = true;
                }
            }
        } else {
            for position in grid.iter() {
                if piste_map[position] == Some(self.id) {
                    piste_map[position] = None
                }
            }
        }

        match pistes.entry(self.id) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().grid = entry.get().grid.paste(&grid);
            }
            Entry::Vacant(entry) => {
                entry.insert(Piste { grid });
            }
        }

        overlay.update(*rectangle);
        selection.clear_selection();
    }
}
