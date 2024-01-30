use commons::geometry::XY;
use commons::grid::{Grid, CORNERS, OFFSETS_8};
use commons::unsafe_ordering::unsafe_ordering;

use crate::model::ability::{Ability, ABILITIES};

pub fn cell_exposure(terrain: &Grid<f32>, position: &XY<u32>) -> f32 {
    terrain
        .offsets(position, &CORNERS)
        .map(|corner| exposure(terrain, &corner))
        .max_by(unsafe_ordering)
        .unwrap_or_default()
}

pub fn cell_difficulty(terrain: &Grid<f32>, position: &XY<u32>) -> Option<Ability> {
    let exposure = cell_exposure(terrain, position);
    ABILITIES
        .iter()
        .find(|ability| exposure <= ability.max_grade())
        .copied()
}

fn grade(terrain: &Grid<f32>, from: &XY<u32>, to: &XY<u32>) -> f32 {
    let fall = terrain[from] - terrain[to];
    let run =
        ((from.x as f32 - to.x as f32).powf(2.0) + (from.y as f32 - to.y as f32).powf(2.0)).sqrt();
    fall / run
}

fn exposure(terrain: &Grid<f32>, from: &XY<u32>) -> f32 {
    terrain
        .offsets(from, &OFFSETS_8)
        .map(|to| grade(terrain, from, &to))
        .max_by(unsafe_ordering)
        .unwrap_or_default()
}
