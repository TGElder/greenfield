use std::collections::HashMap;

use commons::geometry::{xy, XY, XYZ};
use engine::binding::Binding;

use crate::model::lift::Lift;
use crate::model::skier::Skier;
use crate::systems::messenger;

pub struct Parameters<'a> {
    pub mouse_xy: &'a Option<XY<u32>>,
    pub lifts: &'a HashMap<usize, Lift>,
    pub skiers: &'a HashMap<usize, Skier>,
    pub targets: &'a mut HashMap<usize, usize>,
    pub global_targets: &'a mut HashMap<usize, usize>,
    pub messenger: &'a mut messenger::System,
    pub graphics: &'a mut dyn engine::graphics::Graphics,
}

pub fn handle(
    binding: &Binding,
    event: &engine::events::Event,
    Parameters {
        mouse_xy,
        lifts,
        skiers,
        targets,
        global_targets,
        graphics,
        messenger,
    }: Parameters<'_>,
) {
    if !binding.binds_event(event) {
        return;
    }

    let Some(mouse_xy) = mouse_xy else { return };
    let Ok(XYZ { x, y, .. }) = graphics.world_xyz_at(mouse_xy) else {
        return;
    };
    let position = xy(x.round() as u32, y.round() as u32);

    for lift in lifts.values() {
        let pick_up_id = &lift.pick_up.id;
        if lift.pick_up.state.position == position {
            global_targets.clear();

            for &skier_id in skiers.keys() {
                targets.remove(&skier_id);
                global_targets.insert(skier_id, *pick_up_id);
            }

            messenger.send(format!(
                "Global target set to {} for all skiers",
                pick_up_id
            ));

            return;
        }
    }
}
