use commons::color::Rgb;
use commons::geometry::{xyz, XYRectangle, XYZ};

use commons::grid::Grid;
use commons::unsafe_ordering::UnsafeOrderable;
use engine::graphics::elements::Quad;
use engine::graphics::models::cube;
use engine::graphics::transform::{Recolor, Transform};
use engine::graphics::utils::{transformation_matrix, triangles_from_quads, Transformation};
use engine::graphics::Graphics;

use crate::model::building::Building;

const COLOR: Rgb<f32> = Rgb::new(0.447, 0.361, 0.259);

pub fn draw(graphics: &mut dyn Graphics, index: &usize, building: &Building, terrain: &Grid<f32>) {
    let Building { footprint, height } = building;
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
    let total_height = (max_ground_height - min_ground_height) + (*height as f32);

    let origin = xyz(from.x, from.y, min_ground_height);
    let scale = xyz(to.x - from.x, to.y - from.y, total_height);
    let quads = translated_and_scaled_cube(origin + scale / 2.0, scale, COLOR).collect::<Vec<_>>();

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

fn translated_and_scaled_cube(
    translation: XYZ<f32>,
    scale: XYZ<f32>,
    color: Rgb<f32>,
) -> impl Iterator<Item = Quad<Rgb<f32>>> {
    let transformation = transformation_matrix(Transformation {
        translation: Some(translation),
        scale: Some(scale),
        ..Transformation::default()
    });

    cube::model()
        .transform(&transformation)
        .recolor(&|_| color)
        .into_iter()
}
