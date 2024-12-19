use std::collections::HashMap;
use std::f32::consts::PI;

use commons::geometry::{xy, XY, XYZ};
use engine::binding::Binding;
use engine::events::{Button, ButtonState, KeyboardKey};

use crate::model::structure::{Structure, StructureClass};
use crate::services::id_allocator;

pub struct Handler {
    pub enabled: bool,
    pub structures: Vec<usize>,
    pub class: StructureClass,
}

impl Handler {
    pub fn new() -> Handler {
        Handler {
            enabled: false,
            structures: vec![],
            class: StructureClass::ChairliftStation,
        }
    }

    pub fn reset(&mut self) {
        self.enabled = false;
        // self.structures.clear();
        self.class = StructureClass::ChairliftStation;
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
            button: Button::Keyboard(KeyboardKey::String("W".to_string())),
            state: ButtonState::Pressed,
        })
        .binds_event(event)
        {
            self.enabled = !self.enabled;
        }

        if !self.enabled {
            return;
        }

        if (Binding::Single {
            button: Button::Keyboard(KeyboardKey::String("A".to_string())),
            state: ButtonState::Pressed,
        })
        .binds_event(event)
        {
            self.class = match self.class {
                StructureClass::ChairliftStation => StructureClass::ChairliftPylon,
                StructureClass::ChairliftPylon => StructureClass::ChairliftStation,
            }
        }

        if (Binding::Single {
            button: Button::Keyboard(KeyboardKey::String("S".to_string())),
            state: ButtonState::Pressed,
        })
        .binds_event(event)
        {
            self.structures.push(id_allocator.next_id());
        }

        if (Binding::Single {
            button: Button::Keyboard(KeyboardKey::String("Q".to_string())),
            state: ButtonState::Pressed,
        })
        .binds_event(event)
        {
            if let Some(structure) = self
                .structures
                .last()
                .and_then(|structure| structures.get_mut(structure))
            {
                structure.rotation += PI / 4.0;
            }
        }

        if (Binding::Single {
            button: Button::Keyboard(KeyboardKey::String("E".to_string())),
            state: ButtonState::Pressed,
        })
        .binds_event(event)
        {
            if let Some(structure) = self
                .structures
                .last()
                .and_then(|structure| structures.get_mut(structure))
            {
                structure.rotation -= PI / 4.0;
            }
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
                class: self.class,
                position,
                rotation: structures
                    .get(id)
                    .map(|structure| structure.rotation)
                    .unwrap_or_default(),
                under_construction: false,
            },
        );

        if let Some(drawing_id) = drawings.remove(id) {
            graphics.draw_hologram(&drawing_id, &[]).unwrap();
        }
    }
}
