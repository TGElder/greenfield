use std::collections::HashMap;

use commons::geometry::{xy, XYRectangle, XY, XYZ};
use engine::binding::Binding;

use crate::model::Lift;
use crate::systems::overlay;

pub struct Handler {
    pub binding: Binding,
    from: Option<XY<u32>>,
}

impl Handler {
    pub fn new(binding: Binding) -> Handler {
        Handler {
            binding,
            from: None,
        }
    }

    pub fn handle(
        &mut self,
        event: &engine::events::Event,
        lifts: &mut HashMap<usize, Lift>,
        mouse_xy: &Option<XY<u32>>,
        overlay: &mut overlay::System,
        graphics: &mut dyn engine::graphics::Graphics,
    ) {
        if !self.binding.binds_event(event) {
            return;
        }

        let Some(mouse_xy) = mouse_xy else {return};
        let Ok(XYZ { x, y, .. }) = graphics.world_xyz_at(mouse_xy) else {return};
        let position = xy(x.floor() as u32, y.floor() as u32);

        let Some(from) = self.from else {
            self.from = Some(position);
            return;
        };

        let to = position;
        lifts.insert(lifts.len(), Lift { from, to });
        self.from = None;

        overlay.update(XYRectangle { from, to: from });
        overlay.update(XYRectangle { from: to, to });
    }
}
