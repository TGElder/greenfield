use std::collections::{HashMap, HashSet};

use commons::geometry::{xy, XY, XYZ};
use engine::binding::Binding;

use crate::model::gate::Gate;
use crate::systems::global_computer;

pub struct Handler {
    pub binding: Binding,
}

impl Handler {
    pub fn handle(
        &self,
        event: &engine::events::Event,
        mouse_xy: &Option<XY<u32>>,
        gates: &HashMap<usize, Gate>,
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
        let mouse_position = xy(x.round() as u32, y.round() as u32);

        for (gate_id, gate) in gates {
            if gate
                .footprint
                .iter()
                .any(|position| position == mouse_position)
            {
                if open.remove(gate_id) {
                    println!("Gate {} is closed", gate_id);
                } else {
                    open.insert(*gate_id);
                    println!("Gate {} is open", gate_id);
                }

                global_computer.update();
            }
        }
    }
}
