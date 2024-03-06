use commons::geometry::{xyz, XYZ};

use engine::graphics::transform::Transform;
use engine::graphics::utils::{transformation_matrix, triangles_from_quads};
use engine::graphics::Graphics;
use nalgebra::Matrix4;

use crate::draw::model;
use crate::model::frame::{Frame, Model};

pub fn draw(graphics: &mut dyn Graphics, index: &usize, frame: &Frame) {
    let mut transformation =
        transformation_matrix(frame.position, frame.angle, 0.0, 0.0, xyz(1.0, 1.0, 1.0));

    if let Some(XYZ { x, y, z }) = frame.model_offset {
        let offset: Matrix4<f32> = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [x, y, z, 1.0],
        ]
        .into();

        transformation *= offset;
    }

    let quads = match frame.model {
        Model::Standing { skis: false } => model::skier_standing::WITHOUT_SKIS.iter(),
        Model::Standing { skis: true } => model::skier_standing::WITH_SKIS.iter(),
        Model::Sitting => model::skier_sitting::MODEL.iter(),
        Model::Chair => model::chair::MODEL.iter(),
    };
    let transformed_quads = quads
        .copied()
        .collect::<Vec<_>>()
        .transform(&transformation);
    let triangles = triangles_from_quads(&transformed_quads);
    graphics.draw_triangles(index, &triangles).unwrap();
}
