use std::f32::consts::PI;

use commons::color::Rgb;
use commons::geometry::{xyz, XYRectangle};

use commons::grid::Grid;
use commons::unsafe_ordering::UnsafeOrderable;
use engine::graphics::models::cube;
use engine::graphics::transform::{Recolor, Transform};
use engine::graphics::utils::{transformation_matrix, triangles_from_quads, Transformation};
use engine::graphics::{DrawMode, Graphics};

use crate::draw::model::building;
use crate::model::building::{Building, Roof};

const WOOD_COLOR: Rgb<f32> = Rgb::new(0.447, 0.361, 0.259);
const CONCRETE_COLOR: Rgb<f32> = Rgb::new(0.5, 0.5, 0.5);
const ROOF_COLOR: Rgb<f32> = Rgb::new(1.0, 1.0, 1.0);
const UNDER_CONSTRUCTION_COLOR: Rgb<f32> = Rgb::new(0.933, 0.298, 0.008);
const ROOF_HEIGHT: f32 = 0.5;

pub fn draw(graphics: &mut dyn Graphics, index: &usize, building: &Building, terrain: &Grid<f32>) {
    let Building {
        footprint,
        height,
        roof,
        under_construction,
        ..
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

    let peaked_coloring = |&color: &_| {
        if *under_construction {
            UNDER_CONSTRUCTION_COLOR
        } else {
            match color {
                building::Color::Wall(_) => WOOD_COLOR,
                building::Color::GableEnd => WOOD_COLOR,
                building::Color::Roof => ROOF_COLOR,
            }
        }
    };
    let flat_coloring = |&_: &_| {
        if *under_construction {
            UNDER_CONSTRUCTION_COLOR
        } else {
            CONCRETE_COLOR
        }
    };
    let model = match roof {
        Roof::Peaked => building::model(ROOF_HEIGHT, 0.0).recolor(&peaked_coloring),
        Roof::PeakedRotated => building::model(ROOF_HEIGHT, PI / 2.0).recolor(&peaked_coloring),
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

    graphics
        .draw_triangles(index, DrawMode::Solid, &triangles)
        .unwrap();
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
