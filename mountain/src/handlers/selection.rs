use commons::geometry::{xy, XYRectangle, XY, XYZ};
use commons::grid::Grid;
use commons::origin_grid::OriginGrid;
use engine::binding::Binding;
use engine::events::Event;

use crate::systems::overlay;

use super::*;

pub struct Handler {
    pub circle_start: Option<XY<u32>>,
    pub circle_end: Option<XY<u32>>,
    pub target: Option<XY<u32>>,
    pub grid: Option<OriginGrid<bool>>,
    pub bindings: Bindings,
}

pub struct Bindings {
    pub circle_start: Binding,
    pub circle_end: Binding,
    pub clear: Binding,
}

impl Handler {
    pub fn new(bindings: Bindings) -> Handler {
        Handler {
            circle_start: None,
            circle_end: None,
            target: None,
            grid: None,
            bindings,
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
        let previous_rectangle = self.grid.clone();
        if let Event::MouseMoved(_) = event {
            if self.circle_start.is_some() {
                if self.target.is_none() {
                    self.set_circle_end(terrain, mouse_xy, graphics);
                } else {
                    self.set_target(terrain, mouse_xy, graphics);
                }
            }
        }

        if self.bindings.circle_start.binds_event(event) {
            if self.circle_start.is_none() {
                self.set_circle_start(terrain, mouse_xy, graphics);
            } else {
                self.clear_selection();
            }
        }

        if self.bindings.circle_end.binds_event(event) && self.circle_end.is_some() {
            self.set_target(terrain, mouse_xy, graphics);
        }

        self.rasterize();

        let new_rectangle = &self.grid;
        if previous_rectangle != *new_rectangle {
            previous_rectangle
                .iter()
                .chain(new_rectangle.iter())
                .for_each(|rectangle| {
                    overlay.update(XYRectangle {
                        from: *rectangle.origin(),
                        to: *rectangle.origin() + xy(rectangle.width(), rectangle.height()),
                    })
                });
        }
    }

    pub fn clear_selection(&mut self) {
        self.circle_start = None;
        self.circle_end = None;
        self.target = None;
        self.grid = None;
    }

    fn set_circle_start(
        &mut self,
        terrain: &Grid<f32>,
        mouse_xy: &Option<XY<u32>>,
        graphics: &mut dyn Graphics,
    ) {
        let Some(mouse_xy) = mouse_xy else { return };
        let Ok(xyz) = graphics.world_xyz_at(mouse_xy) else {
            return;
        };
        self.circle_start = Some(selected_cell(terrain, xyz));
    }

    fn set_circle_end(
        &mut self,
        terrain: &Grid<f32>,
        mouse_xy: &Option<XY<u32>>,
        graphics: &mut dyn Graphics,
    ) {
        let Some(mouse_xy) = mouse_xy else { return };
        let Ok(xyz) = graphics.world_xyz_at(mouse_xy) else {
            return;
        };
        self.circle_end = Some(selected_cell(terrain, xyz));
    }

    fn set_target(
        &mut self,
        terrain: &Grid<f32>,
        mouse_xy: &Option<XY<u32>>,
        graphics: &mut dyn Graphics,
    ) {
        let Some(mouse_xy) = mouse_xy else { return };
        let Ok(xyz) = graphics.world_xyz_at(mouse_xy) else {
            return;
        };
        self.target = Some(selected_cell(terrain, xyz));
    }

    fn rasterize(&mut self) {
        let Some(circle_start) = self.circle_start else {
            return;
        };
        let Some(circle_end) = self.circle_end else {
            return;
        };
        let circle_start = xy(circle_start.x as f32 + 0.5, circle_start.y as f32 + 0.5);
        let circle_end = xy(circle_end.x as f32 + 0.5, circle_end.y as f32 + 0.5);
        let center = (circle_start + circle_end) / 2.0;
        let radius = (circle_start - circle_end).magnitude() / 2.0;
        let grid = OriginGrid::from_rectangle(
            XYRectangle {
                from: xy(
                    (center.x - radius).floor() as u32,
                    (center.y - radius).floor() as u32,
                ),
                to: xy(
                    (center.x + radius).ceil() as u32,
                    (center.y + radius).ceil() as u32,
                ),
            },
            true,
        );
        let radius_squared = radius.powf(2.0) + 0.25;
        let grid = grid.map(|position, _| {
            ((position.x as f32 + 0.5) - center.x).powf(2.0)
                + ((position.y as f32 + 0.5) - center.y).powf(2.0)
                <= radius_squared
        });
        self.grid = Some(grid)
    }
}

fn selected_cell(terrain: &Grid<f32>, XYZ { x, y, .. }: XYZ<f32>) -> XY<u32> {
    xy(
        (x.floor() as u32).min(terrain.width() - 2),
        (y.floor() as u32).min(terrain.height() - 2),
    )
}
