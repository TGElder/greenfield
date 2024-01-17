use std::f32::consts::SQRT_2;

use commons::geometry::XY;
use commons::grid::{Grid, CORNERS};
use commons::unsafe_ordering::unsafe_ordering;

use crate::model::ability::{Ability, ABILITIES};

pub fn cell_grade(terrain: &Grid<f32>, position: &XY<u32>) -> f32 {
    let corners = &CORNERS
        .iter()
        .map(|offset| terrain.offset(position, offset).unwrap())
        .collect::<Vec<_>>();
    (0..3)
        .flat_map(|from| {
            (from + 1..4).map(move |to| {
                let from_position = corners[from];
                let to_position = corners[to];
                let run = run(&from_position, &to_position);
                let rise = (terrain[from_position] - terrain[to_position]).abs();
                rise / run
            })
        })
        .max_by(unsafe_ordering)
        .unwrap()
}

pub fn cell_difficulty(terrain: &Grid<f32>, position: &XY<u32>) -> Option<Ability> {
    let grade = cell_grade(terrain, position);
    ABILITIES
        .iter()
        .find(|ability| grade <= ability.max_grade())
        .copied()
}

fn run(from: &XY<u32>, to: &XY<u32>) -> f32 {
    if from.x == to.x || from.y == to.y {
        1.0
    } else {
        SQRT_2
    }
}
