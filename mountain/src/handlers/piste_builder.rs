use std::collections::hash_map::Entry;
use std::collections::HashMap;

use commons::geometry::{xy, XYRectangle};
use commons::grid::{Grid, CORNERS_INVERSE};
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

        let id = piste_map[origin].unwrap_or_else(|| id_allocator.next_id());

        // updating piste map

        for cell in rectangle.iter() {
            if add && piste_map[cell].is_none() {
                piste_map[cell] = Some(id)
            } else if subtract && piste_map[cell] == Some(id) {
                piste_map[cell] = None
            }
        }

        // updating piste

        let point_grid = OriginGrid::from_rectangle(
            XYRectangle {
                from: rectangle.from,
                to: xy(rectangle.to.x + 1, rectangle.to.y + 1),
            },
            false,
        );
        let point_grid = point_grid.map(|point, _| {
            piste_map
                .offsets(point, &CORNERS_INVERSE)
                .any(|cell| piste_map[cell] == Some(id))
        });

        match pistes.entry(id) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().grid = entry.get().grid.paste(&point_grid);
            }
            Entry::Vacant(entry) => {
                entry.insert(Piste { grid: point_grid });
            }
        }

        // updating overlay

        overlay.update(rectangle);

        // clearing selection

        selection.clear_selection();
    }
}
