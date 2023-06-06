use commons::geometry::{XY, XYZ};

use crate::binding::Binding;
use crate::{
    engine::Engine,
    events::{Event, EventHandler},
    graphics::Graphics,
};

pub struct Handler {
    mouse_xy: Option<XY<u32>>,
    selection: Option<XYZ<f32>>,
    bindings: Bindings,
}

pub struct Bindings {
    pub start_dragging: Binding,
    pub stop_dragging: Binding,
}

impl Handler {
    pub fn new(bindings: Bindings) -> Handler {
        Handler {
            mouse_xy: None,
            selection: None,
            bindings,
        }
    }
}

impl EventHandler for Handler {
    fn handle(&mut self, event: &Event, _: &mut dyn Engine, graphics: &mut dyn Graphics) {
        if let Event::MouseMoved(xy) = event {
            self.mouse_xy = Some(*xy);
            if let Some(selection) = self.selection {
                graphics.look_at(&selection, xy);
            }
        }

        if self.bindings.start_dragging.binds_event(event) {
            let Some(mouse_xy) = self.mouse_xy else {return};
            if let Ok(xyz) = graphics.world_xyz_at(&mouse_xy) {
                self.selection = Some(xyz)
            }
        }

        if self.bindings.stop_dragging.binds_event(event) {
            self.selection = None;
        }
    }
}
