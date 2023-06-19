use std::collections::hash_map::Entry;
use std::collections::HashMap;

use commons::geometry::XYRectangle;
use commons::origin_grid::OriginGrid;
use engine::binding::Binding;

use crate::model::Piste;
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
        selection: &Option<XYRectangle<u32>>,
        pistes: &mut HashMap<usize, Piste>,
        overlay: &mut overlay::System,
    ) {
        let add = self.bindings.add.binds_event(event);
        let subtract = self.bindings.subtract.binds_event(event);
        if !(add || subtract) {
            return;
        }

        let Some(selection) = selection else {return};
        let grid = OriginGrid::from_rectangle(selection, add);

        match pistes.entry(0) {
            Entry::Vacant(cell) => {
                cell.insert(Piste { grid });
            }
            Entry::Occupied(mut value) => {
                let piste = value.get_mut();
                piste.grid = piste.grid.paste(&grid);
            }
        };

        overlay.update(*selection);
    }
}
