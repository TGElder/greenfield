use commons::geometry::{xy, XYRectangle, XY, XYZ};
use commons::grid::Grid;
use commons::origin_grid::OriginGrid;
use engine::binding::Binding;

use crate::systems::overlay;

use super::*;

pub struct Handler {
    pub cells: Vec<XY<u32>>,
    pub grid: Option<OriginGrid<bool>>,
    pub binding: Bindings,
}

pub struct Bindings {
    pub first_cell: Binding,
    pub second_cell: Binding,
    pub clear: Binding,
}

impl Handler {
    pub fn new(binding: Bindings) -> Handler {
        Handler {
            cells: Vec::with_capacity(3),
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
            self.update_last_cell(terrain, mouse_xy, graphics)
        }

        if self.binding.clear.binds_event(event) && !self.cells.is_empty() {
            self.clear_selection();
        } else if self.binding.first_cell.binds_event(event) && self.cells.is_empty() {
            self.add_cell(terrain, mouse_xy, graphics);
            self.add_cell(terrain, mouse_xy, graphics);
        } else if self.binding.second_cell.binds_event(event) && self.cells.len() == 2 {
            self.add_cell(terrain, mouse_xy, graphics);
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
        self.cells.clear();
    }

    fn add_cell(
        &mut self,
        terrain: &Grid<f32>,
        mouse_xy: &Option<XY<u32>>,
        graphics: &mut dyn Graphics,
    ) {
        let Some(mouse_xy) = mouse_xy else {
            return;
        };
        if let Some(cell) = selected_cell(mouse_xy, graphics, terrain) {
            self.cells.push(cell);
            self.rasterize()
        }
    }

    fn update_last_cell(
        &mut self,
        terrain: &Grid<f32>,
        mouse_xy: &XY<u32>,
        graphics: &mut dyn Graphics,
    ) {
        if let Some(cell) = selected_cell(mouse_xy, graphics, terrain) {
            if let Some(last) = self.cells.last_mut() {
                *last = cell;
                self.rasterize()
            }
        }
    }

    fn rasterize(&mut self) {
        if self.cells.is_empty() {
            return;
        }

        self.grid = Some(OriginGrid::from_rectangle(
            XYRectangle {
                from: xy(
                    self.cells.iter().map(|point| point.x).min().unwrap(),
                    self.cells.iter().map(|point| point.y).min().unwrap(),
                ),
                to: xy(
                    self.cells.iter().map(|point| point.x).max().unwrap(),
                    self.cells.iter().map(|point| point.y).max().unwrap(),
                ),
            },
            true,
        ));
    }
}

fn selected_cell(
    mouse_xy: &XY<u32>,
    graphics: &mut dyn Graphics,
    terrain: &Grid<f32>,
) -> Option<XY<u32>> {
    let Ok(XYZ { x, y, .. }) = graphics.world_xyz_at(mouse_xy) else {
        return None;
    };
    let cell = xy(
        (x.floor() as u32).min(terrain.width() - 2),
        (y.floor() as u32).min(terrain.height() - 2),
    );
    Some(cell)
}
