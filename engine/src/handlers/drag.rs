use commons::geometry::{XY, XYZ};

use crate::binding::Binding;
use crate::{engine::Engine, events::Event, graphics::Graphics};

#[derive(Default)]
pub struct Handler {
    mouse_xy: Option<XY<u32>>,
    selection: Option<XYZ<f32>>,
}

pub struct Bindings {
    pub start_dragging: Binding,
    pub stop_dragging: Binding,
}

impl Handler {
    pub fn handle(
        &mut self,
        bindings: &Bindings,
        event: &Event,
        _: &mut dyn Engine,
        graphics: &mut dyn Graphics,
    ) {
        if let Event::MouseMoved(xy) = event {
            self.mouse_xy = Some(*xy);
            if let Some(selection) = self.selection {
                graphics.look_at(&selection, xy);
            }
        }

        if bindings.start_dragging.binds_event(event) {
            let Some(mouse_xy) = self.mouse_xy else {
                return;
            };
            if let Ok(xyz) = graphics.world_xyz_at(&mouse_xy) {
                self.selection = Some(xyz)
            }
        }

        if bindings.stop_dragging.binds_event(event) {
            self.selection = None;
        }
    }
}
