use commons::geometry::xyz;

use engine::graphics::transform::Transform;
use engine::graphics::utils::{transformation_matrix, triangles_from_quads};
use engine::graphics::Graphics;

use crate::draw::model;
use crate::model::frame::{Frame, Model};

pub fn draw(graphics: &mut dyn Graphics, index: &usize, frame: &Frame) {
    let transformation =
        transformation_matrix(frame.position, frame.angle, 0.0, 0.0, xyz(1.0, 1.0, 1.0));

    let quads = match frame.model {
        Model::Standing { skis: false } => model::skier_standing::model().quads.into_iter(),
        Model::Standing { skis: true } => model::skier_standing::model().quads.into_iter(),
        Model::Sitting => model::skier_sitting::model().quads.into_iter(),
        Model::Chair => model::chair::model().quads.into_iter(),
    };
    let transformed_quads = quads.collect::<Vec<_>>().transform(&transformation);
    let triangles = triangles_from_quads(&transformed_quads);
    graphics.draw_triangles(index, &triangles).unwrap();
}
