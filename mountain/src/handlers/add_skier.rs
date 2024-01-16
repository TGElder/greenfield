use std::collections::HashMap;

use commons::geometry::{xy, XY, XYZ};
use engine::binding::Binding;

use crate::model::direction::Direction;
use crate::model::skiing::{self, Ability};
use crate::services::id_allocator;

pub struct Handler {
    pub bindings: Bindings,
}

pub struct Bindings {
    pub beginner: Binding,
    pub intermediate: Binding,
    pub advanced: Binding,
    pub expert: Binding,
}

impl Handler {
    pub fn handle(
        &self,
        event: &engine::events::Event,
        mouse_xy: &Option<XY<u32>>,
        plans: &mut HashMap<usize, skiing::Plan>,
        abilities: &mut HashMap<usize, Ability>,
        id_allocator: &mut id_allocator::Service,
        graphics: &mut dyn engine::graphics::Graphics,
    ) {
        let ability = if self.bindings.beginner.binds_event(event) {
            Ability::Beginner
        } else if self.bindings.intermediate.binds_event(event) {
            Ability::Intermediate
        } else if self.bindings.advanced.binds_event(event) {
            Ability::Advanced
        } else if self.bindings.expert.binds_event(event) {
            Ability::Expert
        } else {
            return;
        };

        let Some(mouse_xy) = mouse_xy else { return };
        let Ok(XYZ { x, y, .. }) = graphics.world_xyz_at(mouse_xy) else {
            return;
        };

        let id = id_allocator.next_id();

        plans.insert(
            id,
            skiing::Plan::Stationary(skiing::State {
                position: xy(x.round() as u32, y.round() as u32),
                velocity: 1,
                travel_direction: Direction::NorthEast,
            }),
        );

        abilities.insert(id, ability);
    }
}
