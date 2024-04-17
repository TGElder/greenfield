use std::collections::{HashMap, HashSet};

use commons::geometry::{xy, XY, XYZ};
use engine::binding::Binding;

use crate::model::lift::Lift;

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
                    println!("Lift {} is closed", lift_id);
                } else {
                    open.insert(*lift_id);
                    println!("Lift {} is open", lift_id);
                }
            }
        }
    }
}
