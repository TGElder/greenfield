use std::collections::HashMap;

use commons::geometry::{xy, XY, XYZ};
use engine::binding::Binding;

use crate::model::lift::Lift;
use crate::model::skier::Skier;

pub struct Handler {
    pub binding: Binding,
}

impl Handler {
    pub fn handle(
        &self,
        event: &engine::events::Event,
        mouse_xy: &Option<XY<u32>>,
        lifts: &HashMap<usize, Lift>,
        skiers: &HashMap<usize, Skier>,
        global_targets: &mut HashMap<usize, usize>,
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

        for (&lift_id, lift) in lifts {
            if lift.pick_up.state.position == position || lift.drop_off.state.position == position {
                global_targets.clear();

                for &skier_id in skiers.keys() {
                    global_targets.insert(skier_id, lift_id);
                }

                println!("Global target set to {} for all skiers", lift_id);

                return;
            }
        }
    }
}
