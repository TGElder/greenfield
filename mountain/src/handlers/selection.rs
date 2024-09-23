use commons::geometry::{project_point_onto_line, xy, Line, XYRectangle, XY, XYZ};
use commons::grid::{Grid, OFFSETS_4};
use commons::origin_grid::OriginGrid;
use engine::binding::Binding;
use line_drawing::Bresenham;

use crate::model::selection::Selection;
use crate::systems::terrain_artist;

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
    pub terrain_artist: &'a mut terrain_artist::System,
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
            selection,
            graphics,
            terrain_artist,
        }: Parameters<'_>,
    ) {
        if let Event::MouseMoved(mouse_xy) = event {
            self.update_last_cell(terrain, mouse_xy, &mut selection.cells, graphics);
            self.was_clear_interrupted = true;
        }

        if bindings.start_clearing.binds_event(event) {
            self.was_clear_interrupted = false;
        } else if !self.was_clear_interrupted
            && bindings.finish_clearing.binds_event(event)
            && !selection.cells.is_empty()
        {
            selection.cells.clear();
        } else if bindings.first_cell.binds_event(event) && selection.cells.is_empty() {
            self.add_cell(terrain, mouse_xy, &mut selection.cells, graphics);
            self.add_cell(terrain, mouse_xy, &mut selection.cells, graphics);
        } else if bindings.second_cell.binds_event(event) && selection.cells.len() == 2 {
            self.add_cell(terrain, mouse_xy, &mut selection.cells, graphics);
        }

        let previous_grid = selection.grid.clone();

        self.rasterize(terrain, selection);

        let new_grid = &selection.grid;
        if previous_grid != *new_grid {
            previous_grid
                .iter()
                .chain(new_grid.iter())
                .flat_map(|grid| grid.rectangle())
                .for_each(|rectangle| terrain_artist.update_overlay(rectangle));
        }
    }

    fn add_cell(
        &mut self,
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
        &mut self,
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

    fn rasterize(&mut self, terrain: &Grid<f32>, selection: &mut Selection) {
        selection.grid = None;

        let cells = &selection.cells;
        if cells.len() < 2 {
            return;
        }

        // Computing border

        let mut border = Vec::with_capacity(5);

        if cells.len() == 2 {
            border.push(cells[0]);
            border.push(cells[1]);
        } else if cells.len() == 3 {
            border.push(cells[0]);

            let Some(border_1) = self.compute_border_1(&selection.cells) else {
                return;
            };
            if border_1.x > terrain.width() - 2 || border_1.y > terrain.height() {
                return;
            }
            border.push(border_1);

            border.push(cells[2]);

            let border_3 = to_xy_i32(&border[2]) - (to_xy_i32(&border[1]) - to_xy_i32(&border[0]));
            if border_3.x < 0 || border_3.y < 0 {
                return;
            }
            let border_3 = xy(border_3.x as u32, border_3.y as u32);
            if border_3.x > terrain.width() - 2 || border_3.y > terrain.height() {
                return;
            }
            border.push(border_3);

            // Adding fifth cell so final line is picked up by windows(2) below
            border.push(cells[0]);
        }

        // Creating grid

        let mut grid = OriginGrid::from_rectangle(
            XYRectangle {
                from: xy(
                    border.iter().map(|point| point.x).min().unwrap(),
                    border.iter().map(|point| point.y).min().unwrap(),
                ),
                to: xy(
                    border.iter().map(|point| point.x).max().unwrap(),
                    border.iter().map(|point| point.y).max().unwrap(),
                ),
            },
            false,
        );

        // Rasterizing

        for pair in border.windows(2) {
            rasterize_line(&mut grid, &pair[0], &pair[1]);
        }
        let filled = fill_cells_inaccessible_from_border(&grid);

        selection.grid = Some(filled)
    }

    fn compute_border_1(&self, cells: &[XY<u32>]) -> Option<XY<u32>> {
        // Case where user did not drag
        let to = if cells[0] == cells[1] {
            // Default is a grid aligned to the x-axis
            cells[0] + xy(1, 0)
        } else {
            cells[1]
        };

        let out = project_point_onto_line(
            to_xy_f32(&cells[2]),
            Line {
                from: to_xy_f32(&cells[0]),
                to: to_xy_f32(&to),
            },
        );

        let Ok(out) = out else {
            return None;
        };
        let out = xy(out.x.round(), out.y.round());
        if out.x < 0.0 || out.y < 0.0 {
            return None;
        }
        Some(xy(out.x as u32, out.y as u32))
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

fn to_xy_f32(XY { x, y }: &XY<u32>) -> XY<f32> {
    xy(*x as f32, *y as f32)
}

fn to_xy_i32(XY { x, y }: &XY<u32>) -> XY<i32> {
    xy(*x as i32, *y as i32)
}

fn rasterize_line(grid: &mut OriginGrid<bool>, from: &XY<u32>, to: &XY<u32>) {
    for (x, y) in Bresenham::new((from.x as i32, from.y as i32), (to.x as i32, to.y as i32)) {
        grid[xy(x as u32, y as u32)] = true;
    }
}

fn fill_cells_inaccessible_from_border(grid: &OriginGrid<bool>) -> OriginGrid<bool> {
    let border = grid
        .iter()
        .filter(|xy| !grid[xy])
        .filter(|xy| grid.is_border(xy));

    let mut out = grid.map(|_, _| true);
    let mut queue = Vec::with_capacity((grid.width() * grid.height()) as usize);
    for cell in border {
        out[cell] = false;
        queue.push(cell);
    }

    while let Some(cell) = queue.pop() {
        for neighbour in grid.offsets(cell, &OFFSETS_4).collect::<Vec<_>>() {
            if !grid[neighbour] && out[neighbour] {
                out[neighbour] = false;
                queue.push(neighbour);
            }
        }
    }
    out
}
