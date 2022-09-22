pub mod elements;
pub mod matrices;
pub mod projection;
pub mod projections;

pub use projection::Projection;

use elements::*;

pub trait GraphicsBackend {
    fn add_triangles(&mut self, triangles: &[Triangle]) -> usize;

    fn render(&mut self);

    fn screenshot(&self, path: &str);

    fn add_quads(&mut self, quads: &[Quad]) -> usize {
        let triangles = quads
            .iter()
            .flat_map(
                |Quad {
                     id,
                     corners: [a, b, c, d],
                     color,
                 }| {
                    [
                        Triangle {
                            id: *id,
                            corners: [*a, *b, *d],
                            color: *color,
                        },
                        Triangle {
                            id: *id,
                            corners: [*b, *c, *d],
                            color: *color,
                        },
                    ]
                    .into_iter()
                },
            )
            .collect::<Vec<_>>();
        self.add_triangles(&triangles)
    }
}
