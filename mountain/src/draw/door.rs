use commons::color::Rgb;
use commons::geometry::{xyz, XYRectangle, XYZ};

use commons::grid::Grid;
use commons::unsafe_ordering::UnsafeOrderable;
use engine::graphics::elements::Quad;
use engine::graphics::models::cube;
use engine::graphics::transform::{Recolor, Transform};
use engine::graphics::utils::{transformation_matrix, triangles_from_quads, Transformation};
use engine::graphics::Graphics;

use crate::model::direction::Direction;
use crate::model::door::Door;

const STRUCTURE_COLOR: Rgb<f32> = Rgb::new(0.447, 0.361, 0.259);
const APERTURE_COLOR: Rgb<f32> = Rgb::new(0.0, 0.0, 0.0);
const HEIGHT: f32 = 2.0;

pub fn draw(graphics: &mut dyn Graphics, index: &usize, door: &Door, terrain: &Grid<f32>) {
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
    let quads =
        translated_and_scaled_cube(origin + scale / 2.0, scale, &|side| coloring(side, door))
            .collect::<Vec<_>>();

    let triangles = triangles_from_quads(&quads);
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

fn coloring(side: &cube::Side, door: &Door) -> Rgb<f32> {
    if aperture_side(door.direction) == Some(*side) {
        APERTURE_COLOR
    } else {
        STRUCTURE_COLOR
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

fn translated_and_scaled_cube(
    translation: XYZ<f32>,
    scale: XYZ<f32>,
    coloring: &dyn Fn(&cube::Side) -> Rgb<f32>,
) -> impl Iterator<Item = Quad<Rgb<f32>>> {
    let transformation = transformation_matrix(Transformation {
        translation: Some(translation),
        scale: Some(scale),
        ..Transformation::default()
    });

    cube::model()
        .transform(&transformation)
        .recolor(coloring)
        .into_iter()
}
