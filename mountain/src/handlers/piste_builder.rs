use std::collections::HashMap;

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
        selection: &mut selection::Handler,
        overlay: &mut overlay::System,
        id_allocator: &mut id_allocator::Service,
    ) {
        let add = self.bindings.add.binds_event(event) || self.bindings.new.binds_event(event);
        let subtract = self.bindings.subtract.binds_event(event);
        if !(add || subtract) {
            return;
        }

        let Some(rectangle) = selection.selected_rectangle() else {return};
        let grid = OriginGrid::from_rectangle(rectangle, add);

        if pistes.is_empty() || self.bindings.new.binds_event(event) {
            self.id = id_allocator.next_id();
            println!("Building piste {}", self.id);
            pistes.insert(self.id, Piste { grid });
        } else {
            pistes.get_mut(&self.id).unwrap().grid = pistes[&self.id].grid.paste(&grid);
        };

        overlay.update(*rectangle);
        selection.clear_selection();
    }
}
