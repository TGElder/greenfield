use commons::geometry::{project_point_onto_line, xy, Line, XYRectangle, XY, XYZ};
use commons::grid::{Grid, OFFSETS_4};
use commons::origin_grid::OriginGrid;
use engine::binding::Binding;
use line_drawing::Bresenham;

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

    fn rasterize(&mut self, terrain: &Grid<f32>) {
        let cells = &self.cells;
        if cells.len() < 2 {
            return;
        }

        let mut border = Vec::with_capacity(5);

        if cells.len() == 2 {
            border.push(cells[0]);
            border.push(cells[1]);
        } else if cells.len() == 3 {
            if cells[0] == cells[2] {
                border.push(cells[0]);
                border.push(cells[1]);
            } else {
                border.push(cells[0]);
                border.push(self.get_second_rectangle_point());
                border.push(cells[2]);
                border.push(to_xy_u32(
                    &(to_xy_i32(&border[2]) - (to_xy_i32(&border[1]) - to_xy_i32(&border[0]))),
                ));
                border.push(cells[0]);
            }
        }

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

        for pair in border.windows(2) {
            rasterize_line(&mut grid, &pair[0], &pair[1]);
        }

        let filled = fill_cells_inaccessible_from_border(&grid);

        self.grid = Some(filled)
    }

    fn get_second_rectangle_point(&self) -> XY<u32> {
        let cells = &self.cells;

        if cells[0] == cells[2] {
            return cells[1];
        }

        let to = if cells[0] == cells[1] {
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
        let out = out.unwrap();
        xy(out.x.round() as u32, out.y.round() as u32)
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

fn to_xy_u32(XY { x, y }: &XY<i32>) -> XY<u32> {
    xy(*x as u32, *y as u32)
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
