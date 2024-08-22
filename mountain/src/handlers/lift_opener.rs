use std::collections::{HashMap, HashSet};

use commons::geometry::{xy, XY, XYZ};

use crate::handlers::HandlerResult::{self, EventConsumed, EventPersists};
use crate::model::lift::Lift;
use crate::systems::global_computer;

pub fn handle(
    mouse_xy: &Option<XY<u32>>,
    lifts: &HashMap<usize, Lift>,
    open: &mut HashSet<usize>,
    global_computer: &mut global_computer::System,
    graphics: &mut dyn engine::graphics::Graphics,
) -> HandlerResult {
    let Some(mouse_xy) = mouse_xy else {
        return EventPersists;
    };
    let Ok(XYZ { x, y, .. }) = graphics.world_xyz_at(mouse_xy) else {
        return EventPersists;
    };
    let position = xy(x.round() as u32, y.round() as u32);

    let lifts_to_modify = lifts
        .iter()
        .filter(|(_, lift)| {
            lift.pick_up.state.position == position || lift.drop_off.state.position == position
        })
        .collect::<Vec<_>>();

    if lifts_to_modify.is_empty() {
        return EventPersists;
    }

    for (lift_id, lift) in lifts_to_modify {
        if lift.pick_up.state.position == position || lift.drop_off.state.position == position {
            if open.remove(lift_id) {
                println!("Lift {} is closed", lift_id);
                open.remove(&lift.pick_up.id);
                println!("Pick up {} is closed", lift.pick_up.id);
                open.remove(&lift.drop_off.id);
                println!("Drop off {} is closed", lift.drop_off.id);
            } else {
                open.insert(*lift_id);
                println!("Lift {} is open", lift_id);
                open.insert(lift.pick_up.id);
                println!("Pick up {} is open", lift.pick_up.id);
                open.insert(lift.drop_off.id);
                println!("Drop off {} is open", lift.drop_off.id);
            }
        }
    }

    global_computer.update();

    EventConsumed
}
