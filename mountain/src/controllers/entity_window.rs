use std::collections::HashMap;

use commons::geometry::{xy, XY, XYZ};

use crate::controllers::Result::{self, Action, NoAction};
use crate::model::lift::Lift;
use crate::widgets::entity_window::EntityWindow;

pub struct Parameters<'a> {
    pub mouse_xy: &'a Option<XY<u32>>,
    pub lifts: &'a HashMap<usize, Lift>,
    pub windows: &'a mut HashMap<usize, EntityWindow>,
    pub graphics: &'a mut dyn engine::graphics::Graphics,
}

pub fn trigger(
    Parameters {
        mouse_xy,
        lifts,
        windows,
        graphics,
    }: Parameters<'_>,
) -> Result {
    let Some(mouse_xy) = mouse_xy else {
        return Result::NoAction;
    };
    let Ok(XYZ { x, y, .. }) = graphics.world_xyz_at(mouse_xy) else {
        return NoAction;
    };
    let position = xy(x.round() as u32, y.round() as u32);

    for lift in lifts.values() {
        let pick_up_id = &lift.pick_up.id;
        if lift.pick_up.state.position == position {
            windows.insert(*pick_up_id, EntityWindow::new(*pick_up_id));
        }
    }

    Action
}
