use engine::graphics::utils::{transformation_matrix, triangles_from_quads, Transformation};
use engine::graphics::{DrawMode, Graphics};

use crate::draw::model::window::MODEL;
use crate::model::building::Building;

pub struct Drawing {
    pub index: usize,
}

impl Drawing {
    pub fn init(graphics: &mut dyn Graphics) -> Drawing {
        let triangles = triangles_from_quads(&[MODEL]);
        let index = graphics
            .create_instanced_triangles(DrawMode::Solid, &triangles)
            .unwrap();
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
