use std::f32::consts::PI;

use commons::color::Rgb;
use commons::geometry::{xyz, XYRectangle};

use commons::grid::Grid;
use commons::unsafe_ordering::UnsafeOrderable;
use engine::graphics::models::cube;
use engine::graphics::transform::{Recolor, Transform};
use engine::graphics::utils::{transformation_matrix, triangles_from_quads, Transformation};
use engine::graphics::Graphics;

use crate::draw::model::building;
use crate::model::building::{Building, Roof};
use crate::model::direction::Direction;
use crate::model::door::Door;

const WOOD_COLOR: Rgb<f32> = Rgb::new(0.447, 0.361, 0.259);
const CONCRETE_COLOR: Rgb<f32> = Rgb::new(0.5, 0.5, 0.5);
const ROOF_COLOR: Rgb<f32> = Rgb::new(1.0, 1.0, 1.0);
const DOORWAY_COLOR: Rgb<f32> = Rgb::new(0.0, 0.0, 0.0);

const HEIGHT: f32 = 2.0;
const ROOF_HEIGHT: f32 = 0.5;

pub fn draw(
    graphics: &mut dyn Graphics,
    index: &usize,
    door: &Door,
    building: &Building,
    terrain: &Grid<f32>,
) {
    let Door { footprint, .. } = door;
    let XYRectangle { from, to } = footprint;
    let from = xyz(from.x as f32, from.y as f32, terrain[from]);
    let to = xyz(to.x as f32, to.y as f32, terrain[to]);

    let Some((
        UnsafeOrderable {
            value: min_ground_height,
        },
        UnsafeOrderable {
            value: max_ground_height,
        },
    )) = min_max(
        footprint
            .iter()
            .map(|position| terrain[position])
            .map(|value| UnsafeOrderable { value }),
    )
    else {
        return;
    };
    let total_height = (max_ground_height - min_ground_height) + HEIGHT;

    let origin = xyz(from.x, from.y, min_ground_height);
    let scale = xyz(to.x - from.x, to.y - from.y, total_height);

    let peaked_coloring = |&color: &_| match color {
        building::Color::Wall(side) => cube_coloring(&side, door, &WOOD_COLOR),
        building::Color::GableEnd => WOOD_COLOR,
        building::Color::Roof => ROOF_COLOR,
    };
    let flat_coloring = |&side: &_| cube_coloring(&side, door, &CONCRETE_COLOR);
    let model = match building.roof {
        Roof::Peaked | Roof::PeakedRotated => {
            building::model(ROOF_HEIGHT, roof_yaw(door)).recolor(&peaked_coloring)
        }
        Roof::Flat => triangles_from_quads(&cube::model()).recolor(&flat_coloring),
    };

    let triangles = model
        .transform(&transformation_matrix(Transformation {
            translation: Some(origin + scale / 2.0),
            scale: Some(scale),
            ..Transformation::default()
        }))
        .into_iter()
        .collect::<Vec<_>>();

    graphics.draw_triangles(index, &triangles).unwrap();
}

fn min_max<T>(iter: impl Iterator<Item = T>) -> Option<(T, T)>
where
    T: Copy + Ord,
{
    let mut min: Option<T> = None;
    let mut max: Option<T> = None;
    iter.for_each(|value| {
        match min {
            Some(min_value) => min = Some(min_value.min(value)),
            None => min = Some(value),
        }
        match max {
            Some(max_value) => max = Some(max_value.max(value)),
            None => max = Some(value),
        }
    });
    match (min, max) {
        (Some(min), Some(max)) => Some((min, max)),
        _ => None,
    }
}

fn roof_yaw(door: &Door) -> f32 {
    match door.direction {
        Direction::East | Direction::West => PI / 2.0,
        _ => 0.0,
    }
}

fn cube_coloring(side: &cube::Side, door: &Door, wall_color: &Rgb<f32>) -> Rgb<f32> {
    if aperture_side(door.direction) == Some(*side) {
        DOORWAY_COLOR
    } else {
        *wall_color
    }
}

fn aperture_side(direction: Direction) -> Option<cube::Side> {
    match direction {
        Direction::East => Some(cube::Side::Right),
        Direction::North => Some(cube::Side::Front),
        Direction::West => Some(cube::Side::Left),
        Direction::South => Some(cube::Side::Back),
        _ => None,
    }
}
