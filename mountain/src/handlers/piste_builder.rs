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
    pub bindings: Bindings,
}

pub struct Bindings {
    pub add: Binding,
    pub subtract: Binding,
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
        let add = self.bindings.add.binds_event(event);
        let subtract = self.bindings.subtract.binds_event(event);
        if !(add || subtract) {
            return;
        }

        let (Some(origin), Some(rectangle)) = (selection.origin, selection.rectangle) else {
            return;
        };
        let mut grid = OriginGrid::from_rectangle(
            XYRectangle {
                from: rectangle.from,
                to: xy(rectangle.to.x + 1, rectangle.to.y + 1),
            },
            false,
        );

        let id = piste_map[origin].unwrap_or_else(|| id_allocator.next_id());

        if add {
            for position in grid.iter() {
                if piste_map[position].is_none() {
                    grid[position] = true;
                    piste_map[position] = Some(id)
                } else if piste_map[position] == Some(id) {
                    grid[position] = true;
                }
            }
        } else {
            for position in grid.iter() {
                if piste_map[position] == Some(id) {
                    piste_map[position] = None
                }
            }
        }

        match pistes.entry(id) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().grid = entry.get().grid.paste(&grid);
            }
            Entry::Vacant(entry) => {
                entry.insert(Piste { grid });
            }
        }

        overlay.update(XYRectangle {
            from: xy(
                rectangle.from.x.saturating_sub(1),
                rectangle.from.y.saturating_sub(1),
            ),
            to: xy(rectangle.to.x + 1, rectangle.to.y + 1),
        });
        selection.clear_selection();
    }
}
