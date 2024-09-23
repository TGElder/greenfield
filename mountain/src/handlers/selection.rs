use commons::geometry::{xy, XY, XYZ};
use commons::grid::Grid;
use engine::binding::Binding;

use crate::model::selection::Selection;

use super::*;

pub struct Handler {
    pub was_clear_interrupted: bool,
}

pub struct Parameters<'a> {
    pub bindings: &'a Bindings,
    pub mouse_xy: &'a Option<XY<u32>>,
    pub terrain: &'a Grid<f32>,
    pub selection: &'a mut Selection,
    pub graphics: &'a mut dyn engine::graphics::Graphics,
}

pub struct Bindings {
    pub first_cell: Binding,
    pub second_cell: Binding,
    pub start_clearing: Binding,
    pub finish_clearing: Binding,
}

impl Handler {
    pub fn new() -> Handler {
        Handler {
            was_clear_interrupted: false,
        }
    }
    pub fn handle(
        &mut self,
        event: &engine::events::Event,
        Parameters {
            bindings,
            mouse_xy,
            terrain,
            selection: Selection { cells, .. },
            graphics,
        }: Parameters<'_>,
    ) {
        if let Event::MouseMoved(mouse_xy) = event {
            update_last_cell(terrain, mouse_xy, cells, graphics);
            self.was_clear_interrupted = true;
        }

        if bindings.start_clearing.binds_event(event) {
            self.was_clear_interrupted = false;
        } else if !self.was_clear_interrupted
            && bindings.finish_clearing.binds_event(event)
            && !cells.is_empty()
        {
            cells.clear();
        } else if bindings.first_cell.binds_event(event) && cells.is_empty() {
            add_cell(terrain, mouse_xy, cells, graphics);
            add_cell(terrain, mouse_xy, cells, graphics);
        } else if bindings.second_cell.binds_event(event) && cells.len() == 2 {
            add_cell(terrain, mouse_xy, cells, graphics);
        }
    }
}

fn add_cell(
    terrain: &Grid<f32>,
    mouse_xy: &Option<XY<u32>>,
    cells: &mut Vec<XY<u32>>,
    graphics: &mut dyn Graphics,
) {
    let Some(mouse_xy) = mouse_xy else {
        return;
    };
    if let Some(selected_cell) = selected_cell(mouse_xy, graphics, terrain) {
        cells.push(selected_cell);
    }
}

fn update_last_cell(
    terrain: &Grid<f32>,
    mouse_xy: &XY<u32>,
    cells: &mut [XY<u32>],
    graphics: &mut dyn Graphics,
) {
    if let Some(selected_cell) = selected_cell(mouse_xy, graphics, terrain) {
        if let Some(last_cell) = cells.last_mut() {
            *last_cell = selected_cell;
        }
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
