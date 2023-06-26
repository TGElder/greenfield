use std::collections::HashMap;

use commons::origin_grid::OriginGrid;
use engine::binding::Binding;

use crate::handlers::selection;
use crate::model::Piste;
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
        selection: &mut selection::Handler,
        overlay: &mut overlay::System,
        id_allocator: &mut id_allocator::Service,
    ) {
        let add = self.bindings.add.binds_event(event);
        let subtract = self.bindings.subtract.binds_event(event);
        if !(add || subtract) {
            return;
        }

        let Some(rectangle) = selection.selected_rectangle() else {return};
        let grid = OriginGrid::from_rectangle(rectangle, add);

        if pistes.is_empty() {
            pistes.insert(id_allocator.next_id(), Piste { grid });
        } else {
            pistes.iter_mut().for_each(|(_, piste)| {
                piste.grid = piste.grid.paste(&grid);
            });
        };

        overlay.update(*rectangle);
        selection.clear_selection();
    }
}
