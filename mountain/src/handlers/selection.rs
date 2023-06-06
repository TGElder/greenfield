use commons::geometry::{xy, PositionedRectangle, XY, XYZ};
use engine::binding::Binding;

use super::*;

pub struct Handler {
    pub origin: Option<XY<u32>>,
    pub binding: Binding,
}

impl Handler {
    pub fn handle(
        &mut self,
        event: &engine::events::Event,
        mouse_xy: &Option<XY<u32>>,
        selection: &mut Option<PositionedRectangle<u32>>,
        graphics: &mut dyn engine::graphics::Graphics,
    ) {
        if let Event::MouseMoved(mouse_xy) = event {
            self.modify_selection(selection, mouse_xy, graphics)
        }

        if self.binding.binds_event(event) {
            if self.origin.is_none() {
                self.set_origin(mouse_xy, selection, graphics);
            } else {
                self.clear_selection(selection);
            }
        }
    }

    fn set_origin(
        &mut self,
        mouse_xy: &Option<XY<u32>>,
        selection: &mut Option<PositionedRectangle<u32>>,
        graphics: &mut dyn Graphics,
    ) {
        let Some(mouse_xy) = mouse_xy else {return};
        let Ok(xyz) = graphics.world_xyz_at(mouse_xy) else {return};
        let origin = selected_cell(xyz);
        self.origin = Some(origin);
        *selection = Some(PositionedRectangle {
            from: origin,
            to: origin,
        });
    }

    fn modify_selection(
        &self,
        selection: &mut Option<PositionedRectangle<u32>>,
        mouse_xy: &XY<u32>,
        graphics: &mut dyn Graphics,
    ) {
        let Some(origin) = self.origin else {return};
        let Ok(xyz) = graphics.world_xyz_at(mouse_xy) else {return};
        let focus = selected_cell(xyz);

        *selection = Some(PositionedRectangle {
            from: xy(origin.x.min(focus.x), origin.y.min(focus.y)),
            to: xy(origin.x.max(focus.x), origin.y.max(focus.y)),
        });
    }

    fn clear_selection(&mut self, selection: &mut Option<PositionedRectangle<u32>>) {
        self.origin = None;
        *selection = None;
    }
}

fn selected_cell(XYZ { x, y, .. }: XYZ<f32>) -> XY<u32> {
    xy(x.floor() as u32, y.floor() as u32)
}
