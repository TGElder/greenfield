use commons::geometry::xyz;
use engine::graphics::elements::Triangle;
use engine::graphics::models::cube;
use engine::graphics::transform::{Recolor, Transform};
use engine::graphics::utils::{transformation_matrix, triangles_from_quads, Transformation};

use crate::draw::model::prism;

#[derive(Clone, Copy)]
pub enum Color {
    Wall(cube::Side),
    GableEnd,
    Roof,
}

pub fn model(peak_height: f32, roof_yaw: f32) -> Vec<Triangle<Color>> {
    triangles_from_quads(&cube::model().recolor(&|&side| Color::Wall(side)))
        .drain(..)
        .chain(
            prism::model()
                .recolor(&|color| match color {
                    prism::Color::Triangle => Color::GableEnd,
                    prism::Color::Quad => Color::Roof,
                })
                .transform(&transformation_matrix(Transformation {
                    scale: Some(xyz(1.0, 1.0, peak_height)),
                    translation: Some(xyz(0.0, 0.0, 0.5)),
                    yaw: Some(roof_yaw),
                    ..Transformation::default()
                })),
        )
        .collect()
}
