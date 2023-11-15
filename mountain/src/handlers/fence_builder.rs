use commons::geometry::{xy, XY, XYZ};
use commons::grid::Grid;
use engine::binding::Binding;

use crate::model::fence::FencePost;

pub struct Handler {
    pub binding: Binding,
    from: Option<XY<u32>>,
}

pub struct Parameters<'a> {
    pub event: &'a engine::events::Event,
    pub mouse_xy: &'a Option<XY<u32>>,
    pub fences: &'a mut Grid<FencePost>,
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

        // get position

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

        let to = position;
        let delta = xy(to.x as i32 - from.x as i32, to.y as i32 - from.y as i32);
        let fence_position = xy(from.x.max(to.x), from.y.max(to.y));
        match fences[fence_position].toggle_fence(&delta) {
            Ok(is_fence) => println!("Fence from {} to {} = {}", from, to, is_fence),
            Err(error) => println!("{}", error),
        }

        // clearing from position

        self.from = None;
    }
}
