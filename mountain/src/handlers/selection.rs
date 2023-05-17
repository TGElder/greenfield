use commons::geometry::{xy, PositionedRectangle, XY, XYZ};
use engine::events::{ButtonState, KeyboardKey};

use super::*;

pub struct SelectionHandler {
    pub origin: Option<XY<u32>>,
}

impl SelectionHandler {
    pub fn handle(
        &mut self,
        selection: &mut Option<PositionedRectangle<u32>>,
        mouse_xy: &Option<XY<u32>>,
        event: &engine::events::Event,
        graphics: &mut dyn engine::graphics::Graphics,
    ) {
        match event {
            Event::MouseMoved(mouse_xy) => self.modify_selection(selection, mouse_xy, graphics),
            Event::KeyboardInput {
                key: KeyboardKey::X,
                state: ButtonState::Pressed,
            } => {
                if self.origin.is_none() {
                    self.set_origin(selection, mouse_xy, graphics);
                } else {
                    self.clear_selection(selection);
                }
            }
            _ => (),
        }
    }

    fn set_origin(
        &mut self,
        selection: &mut Option<PositionedRectangle<u32>>,
        mouse_xy: &Option<XY<u32>>,
        graphics: &mut dyn Graphics,
    ) {
        let Some(mouse_xy) = mouse_xy else {return};
        let Ok(XYZ { x, y, .. }) = graphics.world_xyz_at(mouse_xy) else {return};
        let origin = xy(x.round() as u32, y.round() as u32);
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
        let Ok(XYZ { x, y, .. }) = graphics.world_xyz_at(mouse_xy) else {return};
        let focus = xy(x.floor() as u32, y.floor() as u32);

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
