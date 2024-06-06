use commons::color::Rgb;
use commons::geometry::xyz;
use engine::graphics::elements::Quad;
use engine::graphics::utils::{transformation_matrix, triangles_from_quads, Transformation};
use engine::graphics::Graphics;

use crate::model::building::Building;

const COLOR: Rgb<f32> = Rgb::new(0.0, 0.0, 0.0);

pub struct Drawing {
    pub index: usize,
}

impl Drawing {
    pub fn init(graphics: &mut dyn Graphics) -> Drawing {
        let quad = Quad {
            corners: [
                xyz(-0.5, -0.01, -0.5),
                xyz(0.5, -0.01, -0.5),
                xyz(0.5, -0.01, 0.5),
                xyz(-0.5, -0.01, 0.5),
            ],
            color: COLOR,
        };
        let triangles = triangles_from_quads(&[quad]);
        let index = graphics.create_instanced_triangles(&triangles).unwrap();
        Drawing { index }
    }

    pub fn draw(
        &self,
        graphics: &mut dyn Graphics,
        buildings: &mut dyn Iterator<Item = &Building>,
    ) {
        let world_matrices = buildings
            .filter(|building| !building.under_construction)
            .flat_map(|building| building.windows.iter())
            .map(|window| Transformation {
                translation: Some(window.position),
                yaw: Some(window.direction.angle()),
                ..Transformation::default()
            })
            .map(transformation_matrix)
            .map(Some)
            .collect::<Vec<_>>();

        graphics
            .update_instanced_triangles(&self.index, &world_matrices)
            .unwrap();
    }
}
