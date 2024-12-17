use std::collections::HashMap;

use commons::geometry::{xy, xyz, XY, XYZ};
use engine::binding::Binding;
use engine::events::{Button, ButtonState, MouseButton};

use crate::model::structure::{Structure, StructureClass};
use crate::services::id_allocator;

pub struct Handler {
    pub structures: Vec<usize>,
}

impl Handler {
    pub fn new() -> Handler {
        Handler { structures: vec![] }
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
        if (Binding::Single {
            button: Button::Mouse(MouseButton::Left),
            state: ButtonState::Pressed,
        })
        .binds_event(event)
        {
            self.structures.push(id_allocator.next_id());
            return;
        }

        if !matches!(event, engine::events::Event::MouseMoved(..)) {
            return;
        }

        if self.structures.is_empty() {
            self.structures.push(id_allocator.next_id());
        }

        let Some(mouse_xy) = mouse_xy else { return };
        let Ok(XYZ { x, y, .. }) = graphics.world_xyz_at(mouse_xy) else {
            return;
        };
        let position = xy(x.round() as u32, y.round() as u32);

        let id = self.structures.last().unwrap();

        structures.insert(
            *id,
            Structure {
                class: StructureClass::ChairliftBaseStation,
                position,
                footprint: xyz(8, 4, 3),
                rotation: 0.0,
                under_construction: false,
                wire_path_out: vec![[xyz(-0.5, -0.5, 0.5), xyz(0.5, -0.5, 0.5)]],
                wire_path_back: vec![[xyz(0.5, 0.5, 0.5), xyz(-0.5, 0.5, 0.5)]],
            },
        );

        if let Some(drawing_id) = drawings.remove(id) {
            graphics.draw_hologram(&drawing_id, &[]).unwrap();
        }
    }
}
