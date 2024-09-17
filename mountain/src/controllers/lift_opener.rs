use std::collections::{HashMap, HashSet};

use commons::geometry::{xy, XY, XYZ};

use crate::controllers::Result::{self, Action, NoAction};
use crate::model::lift::Lift;
use crate::systems::{global_computer, messenger};

pub fn trigger(
    mouse_xy: &Option<XY<u32>>,
    lifts: &HashMap<usize, Lift>,
    open: &mut HashSet<usize>,
    global_computer: &mut global_computer::System,
    messenger: &mut messenger::System,
    graphics: &mut dyn engine::graphics::Graphics,
) -> Result {
    let Some(mouse_xy) = mouse_xy else {
        return NoAction;
    };
    let Ok(XYZ { x, y, .. }) = graphics.world_xyz_at(mouse_xy) else {
        return NoAction;
    };
    let position = xy(x.round() as u32, y.round() as u32);

    let lifts_to_modify = lifts
        .iter()
        .filter(|(_, lift)| {
            lift.pick_up.state.position == position || lift.drop_off.state.position == position
        })
        .collect::<Vec<_>>();

    if lifts_to_modify.is_empty() {
        return NoAction;
    }

    for (lift_id, lift) in lifts_to_modify {
        if lift.pick_up.state.position == position || lift.drop_off.state.position == position {
            if open.remove(lift_id) {
                messenger.send(format!("Lift {} is closed", lift_id));
                open.remove(&lift.pick_up.id);
                messenger.send(format!("Pick up {} is closed", lift.pick_up.id));
                open.remove(&lift.drop_off.id);
                messenger.send(format!("Drop off {} is closed", lift.drop_off.id));
            } else {
                open.insert(*lift_id);
                messenger.send(format!("Lift {} is open", lift_id));
                open.insert(lift.pick_up.id);
                messenger.send(format!("Pick up {} is open", lift.pick_up.id));
                open.insert(lift.drop_off.id);
                messenger.send(format!("Drop off {} is open", lift.drop_off.id));
            }
        }
    }

    global_computer.update();

    Action
}
