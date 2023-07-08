use std::collections::HashMap;

use commons::geometry::{xy, XY, XYZ};
use engine::binding::Binding;

use crate::model::skiing::Mode;
use crate::model::{skiing, Direction};
use crate::services::id_allocator;

pub struct Handler {
    pub binding: Binding,
}

impl Handler {
    pub fn handle(
        &self,
        event: &engine::events::Event,
        mouse_xy: &Option<XY<u32>>,
        plans: &mut HashMap<usize, skiing::Plan>,
        id_allocator: &mut id_allocator::Service,
        graphics: &mut dyn engine::graphics::Graphics,
    ) {
        if !self.binding.binds_event(event) {
            return;
        }

        let Some(mouse_xy) = mouse_xy else {return};
        let Ok(XYZ { x, y, .. }) = graphics.world_xyz_at(mouse_xy) else {return};

        plans.insert(
            id_allocator.next_id(),
            skiing::Plan::Stationary(skiing::State {
                position: xy(x.round() as u32, y.round() as u32),
                mode: Mode::Skiing { velocity: 1 },
                travel_direction: Direction::NorthEast,
            }),
        );
    }
}
