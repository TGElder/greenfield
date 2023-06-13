use std::collections::hash_map::Entry;
use std::collections::HashMap;

use commons::geometry::XYRectangle;
use commons::origin_grid::OriginGrid;
use engine::binding::Binding;

use crate::model::Piste;

pub struct Handler {
    pub binding: Binding,
}

impl Handler {
    pub fn handle(
        &mut self,
        event: &engine::events::Event,
        selection: &Option<XYRectangle<u32>>,
        pistes: &mut HashMap<usize, Piste>,
    ) {
        if !self.binding.binds_event(event) {
            return;
        }

        let Some(selection) = selection else {return};
        let grid = OriginGrid::from_xy_rectangle(selection, true);

        match pistes.entry(0) {
            Entry::Vacant(cell) => {
                cell.insert(Piste { grid });
            }
            Entry::Occupied(mut value) => {
                let piste = value.get_mut();
                piste.grid = piste.grid.paste(&grid);
            }
        };
    }
}
