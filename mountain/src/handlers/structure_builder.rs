use std::collections::HashMap;

use commons::geometry::{xy, xyz, XY, XYZ};

use crate::model::structure::{Structure, StructureClass};
use crate::services::id_allocator;

pub struct Handler {
    structure: Option<usize>,
}

impl Handler {
    pub fn new() -> Handler {
        Handler { structure: None }
    }
    pub fn handle(
        &mut self,
        event: &engine::events::Event,
        mouse_xy: &Option<XY<u32>>,
        id_allocator: &mut id_allocator::Service,
        structures: &mut HashMap<usize, Structure>,
        drawings: &mut HashMap<usize, usize>,
        graphics: &mut dyn engine::graphics::Graphics,
    ) {
        if !matches!(event, engine::events::Event::MouseMoved(..)) {
            return;
        }

        let Some(mouse_xy) = mouse_xy else { return };
        let Ok(XYZ { x, y, .. }) = graphics.world_xyz_at(mouse_xy) else {
            return;
        };
        let position = xy(x.round() as u32, y.round() as u32);

        let id = self.structure.get_or_insert_with(|| id_allocator.next_id());

        structures.insert(
            *id,
            Structure {
                class: StructureClass::ChairliftBaseStation,
                position,
                footprint: xyz(8, 4, 3),
                rotation: 0.0,
                under_construction: false,
            },
        );

        if let Some(drawing_id) = drawings.remove(id) {
            graphics.draw_hologram(&drawing_id, &[]).unwrap();
        }
    }
}
