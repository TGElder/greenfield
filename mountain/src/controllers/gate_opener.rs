use std::collections::{HashMap, HashSet};

use commons::geometry::{xy, XY, XYZ};

use crate::controllers::Result::{self, Action, NoAction};
use crate::model::gate::Gate;
use crate::systems::global_computer;

pub fn trigger(
    mouse_xy: &Option<XY<u32>>,
    gates: &HashMap<usize, Gate>,
    open: &mut HashSet<usize>,
    global_computer: &mut global_computer::System,
    graphics: &mut dyn engine::graphics::Graphics,
) -> Result {
    let Some(mouse_xy) = mouse_xy else {
        return NoAction;
    };
    let Ok(XYZ { x, y, .. }) = graphics.world_xyz_at(mouse_xy) else {
        return NoAction;
    };
    let mouse_position = xy(x.round() as u32, y.round() as u32);

    let gate_ids = gates
        .iter()
        .filter(|(_, gate)| {
            gate.footprint
                .iter()
                .any(|position| position == mouse_position)
        })
        .map(|(gate_id, _)| gate_id)
        .collect::<Vec<_>>();

    if gate_ids.is_empty() {
        return NoAction;
    }

    for gate_id in gate_ids {
        if open.remove(gate_id) {
            println!("Gate {} is closed", gate_id);
        } else {
            open.insert(*gate_id);
            println!("Gate {} is open", gate_id);
        }
    }
    global_computer.update();

    Action
}
