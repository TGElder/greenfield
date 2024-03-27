use commons::color::Rgb;

use crate::glium_backend::graphics::ColoredVertex;
use crate::graphics::elements::Triangle;

pub fn colored_vertices_from_triangles(triangles: &[Triangle<Rgb<f32>>]) -> Vec<ColoredVertex> {
    triangles
        .iter()
        .flat_map(
            |Triangle {
                 corners,
                 normal,
                 color,
             }| {
                corners.iter().map(|corner| ColoredVertex {
                    position: (*corner).into(),
                    normal: (*normal).into(),
                    color: [color.r, color.g, color.b],
                })
            },
        )
        .collect::<Vec<ColoredVertex>>()
}
