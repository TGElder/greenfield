use std::collections::HashMap;

use commons::geometry::{xy, XY, XYZ};
use commons::grid::Grid;

use crate::controllers::Result::{self, Action, NoAction};
use crate::model::reservation::Reservation;
use crate::model::skiing::Plan;
use crate::systems::messenger;

pub struct Parameters<'a> {
    pub mouse_xy: &'a Option<XY<u32>>,
    pub reservations: &'a Grid<HashMap<usize, Reservation>>,
    pub plans: &'a HashMap<usize, Plan>,
    pub locations: &'a HashMap<usize, usize>,
    pub targets: &'a HashMap<usize, usize>,
    pub global_targets: &'a HashMap<usize, usize>,
    pub messenger: &'a mut messenger::System,
    pub graphics: &'a mut dyn engine::graphics::Graphics,
}

pub fn trigger(
    Parameters {
        mouse_xy,
        reservations,
        plans,
        locations,
        targets,
        global_targets,
        messenger,
        graphics,
    }: Parameters<'_>,
) -> Result {
    let Some(mouse_xy) = mouse_xy else {
        return NoAction;
    };
    let Ok(XYZ { x, y, .. }) = graphics.world_xyz_at(mouse_xy) else {
        return NoAction;
    };
    let mouse_position = xy(x.round() as u32, y.round() as u32);

    if reservations[mouse_position].is_empty() {
        return NoAction;
    }

    for (id, _) in reservations[mouse_position].iter() {
        messenger.send(format!("ID = {:?}", id));
        messenger.send(format!("Location = {:?}", locations.get(id)));
        messenger.send(format!("Target = {:?}", targets.get(id)));
        messenger.send(format!("Global target = {:?}", global_targets.get(id)));
        messenger.send(format!("Plan = {:?}", plans.get(id)));
    }

    Action
}
