use commons::geometry::{xy, XYRectangle, XY, XYZ};
use commons::grid::Grid;
use commons::origin_grid::OriginGrid;
use engine::binding::Binding;

use crate::systems::overlay;

use super::*;

pub struct Handler {
    pub origin: Option<XY<u32>>,
    pub grid: Option<OriginGrid<bool>>,
    pub binding: Binding,
}

impl Handler {
    pub fn new(binding: Binding) -> Handler {
        Handler {
            origin: None,
            grid: None,
            binding,
        }
    }
    pub fn handle(
        &mut self,
        event: &engine::events::Event,
        mouse_xy: &Option<XY<u32>>,
        terrain: &Grid<f32>,
        graphics: &mut dyn engine::graphics::Graphics,
        overlay: &mut overlay::System,
    ) {
        let previous_grid = self.grid.clone();
        if let Event::MouseMoved(mouse_xy) = event {
            self.modify_selection(terrain, mouse_xy, graphics)
        }

        if self.binding.binds_event(event) {
            if self.origin.is_none() {
                self.set_origin(terrain, mouse_xy, graphics);
            } else {
                self.clear_selection();
            }
        }

        let new_grid = &self.grid;
        if previous_grid != *new_grid {
            previous_grid
                .iter()
                .chain(new_grid.iter())
                .flat_map(|grid| grid.rectangle())
                .for_each(|rectangle| overlay.update(rectangle));
        }
    }

    pub fn clear_selection(&mut self) {
        self.grid = None;
        self.origin = None;
    }

    fn set_origin(
        &mut self,
        terrain: &Grid<f32>,
        mouse_xy: &Option<XY<u32>>,
        graphics: &mut dyn Graphics,
    ) {
        let Some(mouse_xy) = mouse_xy else { return };
        let Ok(xyz) = graphics.world_xyz_at(mouse_xy) else {
            return;
        };
        let origin = selected_cell(terrain, xyz);
        self.origin = Some(origin);
        self.grid = Some(OriginGrid::from_rectangle(
            XYRectangle {
                from: origin,
                to: origin,
            },
            true,
        ));
    }

    fn modify_selection(
        &mut self,
        terrain: &Grid<f32>,
        mouse_xy: &XY<u32>,
        graphics: &mut dyn Graphics,
    ) {
        let Some(origin) = self.origin else { return };
        let Ok(xyz) = graphics.world_xyz_at(mouse_xy) else {
            return;
        };
        let focus = selected_cell(terrain, xyz);

        self.grid = Some(OriginGrid::from_rectangle(
            XYRectangle {
                from: xy(origin.x.min(focus.x), origin.y.min(focus.y)),
                to: xy(origin.x.max(focus.x), origin.y.max(focus.y)),
            },
            true,
        ));
    }
}

fn selected_cell(terrain: &Grid<f32>, XYZ { x, y, .. }: XYZ<f32>) -> XY<u32> {
    xy(
        (x.floor() as u32).min(terrain.width() - 2),
        (y.floor() as u32).min(terrain.height() - 2),
    )
}
