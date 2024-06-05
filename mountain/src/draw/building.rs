use std::f32::consts::PI;

use commons::color::Rgb;
use commons::geometry::{xyz, XYRectangle};

use commons::grid::Grid;
use commons::unsafe_ordering::UnsafeOrderable;
use engine::graphics::transform::{Recolor, Transform};
use engine::graphics::utils::{transformation_matrix, Transformation};
use engine::graphics::Graphics;

use crate::draw::model::building;
use crate::model::building::{Building, Roof};

const WALL_COLOR: Rgb<f32> = Rgb::new(0.447, 0.361, 0.259);
const ROOF_COLOR: Rgb<f32> = Rgb::new(1.0, 1.0, 1.0);
const ROOF_HEIGHT: f32 = 0.5;

pub fn draw(graphics: &mut dyn Graphics, index: &usize, building: &Building, terrain: &Grid<f32>) {
    let Building {
        footprint,
        height,
        roof,
    } = building;
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
    let roof_yaw = match roof {
        Roof::Default => 0.0,
        Roof::Rotated => PI / 2.0,
    };

    let triangles = building::model(ROOF_HEIGHT, roof_yaw)
        .transform(&transformation_matrix(Transformation {
            translation: Some(origin + scale / 2.0),
            scale: Some(scale),
            ..Transformation::default()
        }))
        .recolor(&|color| match color {
            building::Color::Wall(_) => WALL_COLOR,
            building::Color::GableEnd => WALL_COLOR,
            building::Color::Roof => ROOF_COLOR,
        })
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
