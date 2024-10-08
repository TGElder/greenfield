use commons::geometry::{project_point_onto_line, xy, Line, XYRectangle, XY};
use commons::grid::{Grid, OFFSETS_4};
use commons::origin_grid::OriginGrid;
use line_drawing::Bresenham;

use crate::model::selection::Selection;
use crate::systems::terrain_artist;

pub struct Parameters<'a> {
    pub terrain: &'a Grid<f32>,
    pub selection: &'a mut Selection,
    pub terrain_artist: &'a mut terrain_artist::System,
}

pub fn run(
    Parameters {
        terrain,
        selection,
        terrain_artist,
    }: Parameters<'_>,
) {
    let previous_grid = selection.grid.clone();

    selection.grid = rasterize(terrain, selection);
    if previous_grid != selection.grid {
        previous_grid
            .iter()
            .chain(selection.grid.iter())
            .flat_map(|grid| grid.rectangle())
            .for_each(|rectangle| terrain_artist.update_overlay(rectangle));
    }
}

fn rasterize(terrain: &Grid<f32>, selection: &mut Selection) -> Option<OriginGrid<bool>> {
    let cells = &selection.cells;
    if cells.len() < 2 {
        return None;
    }

    // Computing border

    let mut border = Vec::with_capacity(5);

    if cells.len() == 2 {
        border.push(cells[0]);
        border.push(cells[1]);
    } else if cells.len() == 3 {
        border.push(cells[0]);

        let border_1 = compute_border_1(&selection.cells)?;
        if border_1.x > terrain.width() - 2 || border_1.y > terrain.height() {
            return None;
        }
        border.push(border_1);

        border.push(cells[2]);

        let border_3 = to_xy_i32(&border[2]) - (to_xy_i32(&border[1]) - to_xy_i32(&border[0]));
        if border_3.x < 0 || border_3.y < 0 {
            return None;
        }
        let border_3 = xy(border_3.x as u32, border_3.y as u32);
        if border_3.x > terrain.width() - 2 || border_3.y > terrain.height() {
            return None;
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

    Some(filled)
}

fn to_xy_i32(XY { x, y }: &XY<u32>) -> XY<i32> {
    xy(*x as i32, *y as i32)
}

fn to_xy_f32(XY { x, y }: &XY<u32>) -> XY<f32> {
    xy(*x as f32, *y as f32)
}

fn compute_border_1(cells: &[XY<u32>]) -> Option<XY<u32>> {
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
