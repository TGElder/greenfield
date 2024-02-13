use crate::graphics::{Quad, TexturedPosition, Triangle};

pub fn triangles_from_quads(quads: &[Quad]) -> Vec<Triangle> {
    let triangles = quads
        .iter()
        .flat_map(
            |Quad {
                 corners: [a, b, c, d],
                 color,
             }| {
                [
                    Triangle {
                        corners: [*a, *b, *d],
                        color: *color,
                    },
                    Triangle {
                        corners: [*b, *c, *d],
                        color: *color,
                    },
                ]
                .into_iter()
            },
        )
        .collect::<Vec<_>>();
    triangles
}

pub fn textured_triangles_from_textured_quads(
    quads: &[[TexturedPosition; 4]],
) -> Vec<[TexturedPosition; 3]> {
    quads
        .iter()
        .flat_map(|[a, b, c, d]| [[*a, *b, *d], [*b, *c, *d]].into_iter())
        .collect()
}
