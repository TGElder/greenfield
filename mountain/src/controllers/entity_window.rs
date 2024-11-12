use std::collections::HashMap;

use commons::geometry::{xy, XY, XYZ};

use crate::controllers::Result::{self, Action, NoAction};
use crate::model::gate::Gate;
use crate::model::lift::Lift;
use crate::model::piste::Piste;
use crate::widgets::entity_window::EntityWindow;

pub struct Parameters<'a> {
    pub mouse_xy: &'a Option<XY<u32>>,
    pub lifts: &'a HashMap<usize, Lift>,
    pub gates: &'a HashMap<usize, Gate>,
    pub pistes: &'a HashMap<usize, Piste>,
    pub windows: &'a mut HashMap<usize, EntityWindow>,
    pub graphics: &'a mut dyn engine::graphics::Graphics,
}

pub fn trigger(
    Parameters {
        mouse_xy,
        lifts,
        gates,
        pistes,
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

    for (lift_id, lift) in lifts.iter() {
        if lift.pick_up.state.position == position || lift.drop_off.state.position == position {
            windows.insert(*lift_id, EntityWindow::new(*lift_id, *mouse_xy));
            return Action;
        }
    }

    for (gate_id, gate) in gates.iter() {
        if gate.footprint.contains(position) {
            windows.insert(*gate_id, EntityWindow::new(*gate_id, *mouse_xy));
            return Action;
        }
    }

    for (piste_id, piste) in pistes.iter() {
        if piste.grid.in_bounds(position) && piste.grid[position] {
            windows.insert(*piste_id, EntityWindow::new(*piste_id, *mouse_xy));
            return Action;
        }
    }

    Action
}
