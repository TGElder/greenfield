use commons::geometry::{xy, XYRectangle, XY, XYZ};
use commons::grid::{Grid, OFFSETS_4};
use commons::origin_grid::OriginGrid;
use engine::binding::Binding;
use engine::events::Event;
use line_drawing::Bresenham;

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
        if let Some(target) = self.target {
            let b = project_point_onto_line(&target, (&circle_start, &circle_end));
            let d = xy(target.x as i32, target.y as i32)
                - (xy(b.x as i32, b.y as i32) - xy(circle_start.x as i32, circle_start.y as i32));
            let d = xy(d.x as u32, d.y as u32);
            let mut grid = OriginGrid::from_rectangle(
                XYRectangle {
                    from: xy(
                        circle_start.x.min(b.x).min(target.x).min(d.x),
                        circle_start.y.min(b.y).min(target.y).min(d.y),
                    ),
                    to: xy(
                        circle_start.x.max(b.x).max(target.x).max(d.x),
                        circle_start.y.max(b.y).max(target.y).max(d.y),
                    ),
                },
                false,
            );
            for (x, y) in Bresenham::new(
                (circle_start.x as i32, circle_start.y as i32),
                (b.x as i32, b.y as i32),
            ) {
                let position = xy(x as u32, y as u32);
                grid[position] = true;
            }
            for (x, y) in
                Bresenham::new((b.x as i32, b.y as i32), (target.x as i32, target.y as i32))
            {
                let position = xy(x as u32, y as u32);
                grid[position] = true;
            }
            for (x, y) in
                Bresenham::new((target.x as i32, target.y as i32), (d.x as i32, d.y as i32))
            {
                let position = xy(x as u32, y as u32);
                grid[position] = true;
            }
            for (x, y) in Bresenham::new(
                (d.x as i32, d.y as i32),
                (circle_start.x as i32, circle_start.y as i32),
            ) {
                let position = xy(x as u32, y as u32);
                grid[position] = true;
            }
            let border = grid.iter().filter(|xy| grid.is_border(xy) && !grid[xy]).collect::<Vec<_>>();
            self.grid = Some(flood_fill(&border, &grid));
        } else {
            let mut grid = OriginGrid::from_rectangle(
                XYRectangle {
                    from: xy(
                        circle_start.x.min(circle_end.x),
                        circle_start.y.min(circle_end.y),
                    ),
                    to: xy(
                        circle_start.x.max(circle_end.x),
                        circle_start.y.max(circle_end.y),
                    ),
                },
                false,
            );
            for (x, y) in Bresenham::new(
                (circle_start.x as i32, circle_start.y as i32),
                (circle_end.x as i32, circle_end.y as i32),
            ) {
                let position = xy(x as u32, y as u32);
                grid[position] = true;
            }
            self.grid = Some(grid)
        };
    }
}

fn selected_cell(terrain: &Grid<f32>, XYZ { x, y, .. }: XYZ<f32>) -> XY<u32> {
    xy(
        (x.floor() as u32).min(terrain.width() - 2),
        (y.floor() as u32).min(terrain.height() - 2),
    )
}

fn project_point_onto_line(point: &XY<u32>, (from, to): (&XY<u32>, &XY<u32>)) -> XY<u32> {
    let e1 = xy(to.x as f32 - from.x as f32, to.y as f32 - from.y as f32);
    let e2 = xy(
        point.x as f32 - from.x as f32,
        point.y as f32 - from.y as f32,
    );
    let dot_product = e1.x * e2.x + e1.y * e2.y;
    let cos_a = dot_product / (e1.magnitude() * e2.magnitude());
    let v1_p_magnitude = cos_a * e2.magnitude();
    let p = v1_p_magnitude / e1.magnitude();
    let out = xy(from.x as f32 + p * e1.x, from.y as f32 + p * e1.y);
    xy(out.x.round() as u32, out.y.round() as u32)
}

fn flood_fill(positions: &[XY<u32>], grid: &OriginGrid<bool>) -> OriginGrid<bool> {
    let mut out = grid.map(|_, _| true);
    let mut queue = Vec::with_capacity((grid.width() * grid.height()) as usize);
    for position in positions {
        out[position] = false;
        queue.push(*position);
    }
    while let Some(position) = queue.pop() {
        for corner in grid.offsets(position, &OFFSETS_4).collect::<Vec<_>>() {
            if !grid[corner] && out[corner] {
                out[corner] = false;
                queue.push(corner);
            }
        }
    }

    out
}
