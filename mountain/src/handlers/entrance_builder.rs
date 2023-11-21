use std::collections::HashMap;

use commons::geometry::{xy, XY, XYZ};
use commons::grid::Grid;
use engine::binding::Binding;

use crate::model::entrance::Entrance;

pub struct Handler {
    pub binding: Binding,
    from: Option<XY<u32>>,
}

pub struct Parameters<'a> {
    pub event: &'a engine::events::Event,
    pub mouse_xy: &'a Option<XY<u32>>,
    pub piste_map: &'a Grid<Option<usize>>,
    pub entrances: &'a mut HashMap<usize, Entrance>,
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
            piste_map,
            entrances,
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
        let position = xy(x.round() as u32, y.round() as u32);

        // handle case where from position is not set

        let Some(from) = self.from else {
            self.from = Some(position);
            return;
        };

        // clearing from position

        self.from = None;

        // create fence

        let to = position;
        if from.x != to.x && from.y != to.y {
            println!("WARN: Entrance must be horizontal");
            return;
        }
        if from.x == to.x && from.y == to.y {
            println!("WARN: Entrance must not be zero length");
            return;
        }

        if from.x == to.x {
            if from.x == 0 {
                println!("WARN: Entrance must not be against edge of map"); // TODO other edge
                return;
            }

            let min_y = from.y.min(to.y);
            let max_y = from.y.max(to.y);

            let mut left = None;
            let mut right = None;

            for y in min_y..max_y {
                let focus_left = piste_map[xy(from.x - 1, y)];
                if focus_left.is_none() {
                    println!("WARN: Entrance must be adjacent to pistes");
                    return;
                }
                if let Some(left) = left {
                    if left != focus_left {
                        println!("WARN: Adjacent pistes must be the same across entire entrance");
                        return;
                    }
                } else {
                    left = Some(focus_left);
                }

                let focus_right = piste_map[xy(from.x, y)];
                if focus_right.is_none() {
                    println!("WARN: Entrance must be adjacent to pistes");
                    return;
                }
                if let Some(right) = right {
                    if right != focus_right {
                        println!("WARN: Adjacent pistes must be the same across entire entrance");
                        return;
                    }
                } else {
                    right = Some(focus_right);
                }
            }
        }

        if from.y == to.y {
            if from.y == 0 {
                println!("WARN: Entrance must not be against edge of map"); // TODO other edge
                return;
            }

            let min_y = from.x.min(to.x);
            let max_y = from.x.max(to.x);

            let mut above = None;
            let mut below = None;

            for x in min_y..max_y {
                let focus_above = piste_map[xy(x, from.y - 1)];
                if focus_above.is_none() {
                    println!("WARN: Entrance must be adjacent to pistes");
                    return;
                }
                if let Some(above) = above {
                    if above != focus_above {
                        println!("WARN: Adjacent pistes must be the same across entire entrance");
                        return;
                    }
                } else {
                    above = Some(focus_above);
                }

                let focus_below = piste_map[xy(x, from.y)];
                if focus_below.is_none() {
                    println!("WARN: Entrance must be adjacent to pistes");
                    return;
                }
                if let Some(below) = below {
                    if below != focus_below {
                        println!("WARN: Adjacent pistes must be the same across entire entrance");
                        return;
                    }
                } else {
                    below = Some(focus_below);
                }
            }
        }
    }
}
