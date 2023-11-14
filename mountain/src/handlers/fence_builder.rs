use commons::geometry::{xy, XY, XYZ};
use commons::grid::Grid;
use engine::binding::Binding;

use crate::model::fences::Fences;

pub struct Handler {
    pub binding: Binding,
    from: Option<XY<u32>>,
}

pub struct Parameters<'a> {
    pub event: &'a engine::events::Event,
    pub mouse_xy: &'a Option<XY<u32>>,
    pub fences: &'a mut Grid<Fences>,
    pub graphics: &'a mut dyn engine::graphics::Graphics,
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
        Parameters {
            event,
            mouse_xy,
            fences,
            graphics,
        }: Parameters<'_>,
    ) {
        if !self.binding.binds_event(event) {
            return;
        }

        let Some(mouse_xy) = mouse_xy else { return };
        let Ok(XYZ { x, y, .. }) = graphics.world_xyz_at(mouse_xy) else {
            return;
        };
        let position = xy(x.floor() as u32, y.floor() as u32);

        // handle case where from position is not set

        let Some(from) = self.from else {
            self.from = Some(position);
            return;
        };

        // create fence

        self.from = None;
        let to = position;

        if !((from.x == to.x && from.y.abs_diff(to.y) == 1)
            || (from.y == to.y && from.x.abs_diff(to.x) == 1))
        {
            return;
        }

        let delta = xy(to.x as i32 - from.x as i32, to.y as i32 - from.y as i32);
        let fence_index = match delta {
            XY { x: 1, y: 0 } => 0,
            XY { x: 0, y: 1 } => 1,
            XY { x: -1, y: 0 } => 2,
            XY { x: 0, y: -1 } => 3,
            _ => panic!("Unexpected delta {}", delta),
        };
        let fence_position = xy(from.x.max(to.x), from.y.max(to.y));
        let present = &mut fences[fence_position].present[fence_index];
        *present = !*present;

        println!("Fence from {} to {} = {}", from, to, present);
    }
}
