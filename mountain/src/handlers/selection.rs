use commons::geometry::{xy, XYRectangle, XY, XYZ};
use engine::binding::Binding;

use crate::systems::overlay;

use super::*;

pub struct Handler {
    pub origin: Option<XY<u32>>,
    pub rectangle: Option<XYRectangle<u32>>,
    pub binding: Binding,
}

impl Handler {
    pub fn new(binding: Binding) -> Handler {
        Handler {
            origin: None,
            rectangle: None,
            binding,
        }
    }
    pub fn handle(
        &mut self,
        event: &engine::events::Event,
        mouse_xy: &Option<XY<u32>>,
        graphics: &mut dyn engine::graphics::Graphics,
        overlay: &mut overlay::System,
    ) {
        let previous_rectangle = self.rectangle;

        if let Event::MouseMoved(mouse_xy) = event {
            self.modify_selection(mouse_xy, graphics)
        }

        if self.binding.binds_event(event) {
            if self.origin.is_none() {
                self.set_origin(mouse_xy, graphics);
            } else {
                self.clear_selection();
            }
        }

        let new_rectangle = self.rectangle;
        if previous_rectangle != new_rectangle {
            previous_rectangle
                .iter()
                .for_each(|update| overlay.update(*update));
            new_rectangle
                .iter()
                .for_each(|update| overlay.update(*update));
        }
    }

    pub fn selected_rectangle(&self) -> &Option<XYRectangle<u32>> {
        &self.rectangle
    }

    pub fn clear_selection(&mut self) {
        self.rectangle = None;
        self.origin = None;
    }

    fn set_origin(&mut self, mouse_xy: &Option<XY<u32>>, graphics: &mut dyn Graphics) {
        let Some(mouse_xy) = mouse_xy else {return};
        let Ok(xyz) = graphics.world_xyz_at(mouse_xy) else {return};
        let origin = selected_cell(xyz);
        self.origin = Some(origin);
        self.rectangle = Some(XYRectangle {
            from: origin,
            to: origin,
        });
    }

    fn modify_selection(&mut self, mouse_xy: &XY<u32>, graphics: &mut dyn Graphics) {
        let Some(origin) = self.origin else {return};
        let Ok(xyz) = graphics.world_xyz_at(mouse_xy) else {return};
        let focus = selected_cell(xyz);

        self.rectangle = Some(XYRectangle {
            from: xy(origin.x.min(focus.x), origin.y.min(focus.y)),
            to: xy(origin.x.max(focus.x), origin.y.max(focus.y)),
        });
    }
}

fn selected_cell(XYZ { x, y, .. }: XYZ<f32>) -> XY<u32> {
    xy(x.floor() as u32, y.floor() as u32)
}
