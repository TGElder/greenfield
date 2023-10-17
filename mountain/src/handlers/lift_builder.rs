use std::collections::HashMap;

use commons::geometry::{xy, xyz, XYRectangle, XY, XYZ};
use commons::grid::Grid;
use engine::binding::Binding;

use crate::model::car::Car;
use crate::model::lift::Lift;
use crate::services::id_allocator;
use crate::systems::overlay;

pub const LIFT_VELOCITY: f32 = 2.0;

pub struct Handler {
    pub binding: Binding,
    from: Option<XY<u32>>,
}

impl Handler {
    pub fn new(binding: Binding) -> Handler {
        Handler {
            binding,
            from: None,
        }
    }

    pub fn handle(
        &mut self,
        event: &engine::events::Event,
        mouse_xy: &Option<XY<u32>>,
        terrain: &Grid<f32>,
        lifts: &mut HashMap<usize, Lift>,
        overlay: &mut overlay::System,
        id_allocator: &mut id_allocator::Service,
        cars: &mut HashMap<usize, Car>,
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

        let Some(from) = self.from else {
            self.from = Some(position);
            return;
        };

        let to = position;
        let lift_id = id_allocator.next_id();
        lifts.insert(
            lift_id,
            Lift {
                from,
                to,
                velocity: LIFT_VELOCITY,
            },
        );
        self.from = None;

        overlay.update(XYRectangle { from, to: from });
        overlay.update(XYRectangle { from: to, to });

        // cars
        let from = xyz(from.x as f32, from.y as f32, terrain[from]);
        let to = xyz(to.x as f32, to.y as f32, terrain[to]);

        let length =
            ((from.x - to.x).powf(2.0) + (from.y - to.y).powf(2.0) + (from.z - to.z)).sqrt();

        let mut position = 0.0;
        while position < length * 2.0 {
            position += 10.0;
            cars.insert(
                id_allocator.next_id(),
                Car {
                    position,
                    lift: lift_id,
                },
            );
        }
    }
}
