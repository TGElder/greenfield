use std::collections::HashMap;

use commons::geometry::{xy, XY, XYZ};
use commons::grid::Grid;
use engine::binding::Binding;

use crate::handlers::HandlerResult::{self, EventConsumed, EventPersists};
use crate::model::reservation::Reservation;
use crate::model::skiing::Plan;

pub struct Handler {
    pub binding: Binding,
}

pub struct Parameters<'a> {
    pub mouse_xy: &'a Option<XY<u32>>,
    pub reservations: &'a Grid<HashMap<usize, Reservation>>,
    pub plans: &'a HashMap<usize, Plan>,
    pub locations: &'a HashMap<usize, usize>,
    pub targets: &'a HashMap<usize, usize>,
    pub global_targets: &'a HashMap<usize, usize>,
    pub graphics: &'a mut dyn engine::graphics::Graphics,
}

impl Handler {
    pub fn handle(
        &self,
        event: &engine::events::Event,
        Parameters {
            mouse_xy,
            reservations,
            plans,
            locations,
            targets,
            global_targets,
            graphics,
        }: Parameters<'_>,
    ) -> HandlerResult {
        if !self.binding.binds_event(event) {
            return EventPersists;
        }

        let Some(mouse_xy) = mouse_xy else {
            return EventPersists;
        };
        let Ok(XYZ { x, y, .. }) = graphics.world_xyz_at(mouse_xy) else {
            return EventPersists;
        };
        let mouse_position = xy(x.round() as u32, y.round() as u32);

        if reservations[mouse_position].is_empty() {
            return EventPersists;
        }

        for (id, _) in reservations[mouse_position].iter() {
            println!("ID = {:?}", id);
            println!("Location = {:?}", locations.get(id));
            println!("Target = {:?}", targets.get(id));
            println!("Global target = {:?}", global_targets.get(id));
            println!("Plan = {:?}", plans.get(id));
        }

        EventConsumed
    }
}
