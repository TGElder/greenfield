use std::collections::{HashMap, HashSet};

use commons::geometry::{xy, XY, XYZ};
use engine::binding::Binding;

use crate::model::lift::Lift;
use crate::systems::global_computer;

pub struct Handler {
    pub binding: Binding,
}

impl Handler {
    pub fn handle(
        &self,
        event: &engine::events::Event,
        mouse_xy: &Option<XY<u32>>,
        lifts: &HashMap<usize, Lift>,
        open: &mut HashSet<usize>,
        global_computer: &mut global_computer::System,
        graphics: &mut dyn engine::graphics::Graphics,
    ) {
        if !self.binding.binds_event(event) {
            return;
        }

        let Some(mouse_xy) = mouse_xy else { return };
        let Ok(XYZ { x, y, .. }) = graphics.world_xyz_at(mouse_xy) else {
            return;
        };
        let position = xy(x.round() as u32, y.round() as u32);

        for (lift_id, lift) in lifts {
            if lift.pick_up.state.position == position || lift.drop_off.state.position == position {
                if open.remove(lift_id) {
                    println!("INFO: Lift {} is closed", lift_id);
                    open.remove(&lift.pick_up.id);
                    println!("INFO: Pick up {} is closed", lift.pick_up.id);
                    open.remove(&lift.drop_off.id);
                    println!("INFO: Drop off {} is closed", lift.drop_off.id);
                } else {
                    open.insert(*lift_id);
                    println!("INFO: Lift {} is open", lift_id);
                    open.insert(lift.pick_up.id);
                    println!("INFO: Pick up {} is open", lift.pick_up.id);
                    open.insert(lift.drop_off.id);
                    println!("INFO: Drop off {} is open", lift.drop_off.id);
                }

                global_computer.update();
            }
        }
    }
}
