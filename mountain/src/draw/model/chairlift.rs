use commons::color::Rgb;
use commons::geometry::xyz;
use engine::graphics::elements::Triangle;
use engine::graphics::models::cube;
use engine::graphics::transform::{Recolor, Transform};
use engine::graphics::utils::{transformation_matrix, triangles_from_quads, Transformation};

use crate::draw::model::prism;

const COLOR: Rgb<f32> = Rgb::new(0.25, 0.25, 0.25);

pub fn pylon() -> Vec<Triangle<Rgb<f32>>> {
    let mut tower = prism::model()
        .transform(&transformation_matrix(Transformation {
            translation: Some(xyz(0.0, 0.0, 0.0)),
            scale: Some(xyz(0.5, 0.25, 1.0)),
            ..Transformation::default()
        }))
        .recolor(&|_| COLOR);
    let mut hanger = triangles_from_quads(
        &cube::model()
            .transform(&transformation_matrix(Transformation {
                translation: Some(xyz(0.0, 0.0, 1.0)),
                scale: Some(xyz(0.25, 1.0, 0.05)),
                ..Transformation::default()
            }))
            .recolor(&|_| COLOR),
    );
    tower.drain(..).chain(hanger.drain(..)).collect::<Vec<_>>()
}

pub fn station() -> Vec<Triangle<Rgb<f32>>> {
    triangles_from_quads(
        &cube::model()
            .transform(&transformation_matrix(Transformation {
                translation: Some(xyz(0.0, 0.0, 3.5)),
                ..Transformation::default()
            }))
            .recolor(&|_| COLOR),
    )
}
