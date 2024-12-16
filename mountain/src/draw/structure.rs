use commons::geometry::xyz;
use commons::grid::Grid;
use engine::graphics::transform::Transform;
use engine::graphics::utils::{transformation_matrix, triangles_from_quads, Transformation};
use engine::graphics::Graphics;

use crate::draw::model;
use crate::model::structure::{Structure, StructureClass};

pub fn draw(
    graphics: &mut dyn Graphics,
    index: &usize,
    structure: &Structure,
    terrain: &Grid<f32>,
) {
    let model = match structure.class {
        StructureClass::ChairliftBaseStation => model::chairlift::base_station(),
    };
    let quads = model
        .quads
        .transform(&transformation_matrix(Transformation {
            translation: Some(xyz(
                structure.position.x as f32,
                structure.position.y as f32,
                terrain[structure.position],
            )),
            scale: Some(xyz(
                structure.footprint.x as f32,
                structure.footprint.y as f32,
                structure.footprint.z as f32,
            )),
            yaw: Some(structure.rotation),
            ..Transformation::default()
        }));
    let triangles = triangles_from_quads(&quads);
    graphics.draw_hologram(index, &triangles).unwrap();
}
