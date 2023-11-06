mod chair;
mod skier_sitting;
mod skier_standing;

use commons::geometry::XYZ;

use engine::graphics::transform::Transform;
use engine::graphics::Graphics;
use nalgebra::Matrix4;

use crate::model::frame::{Frame, Model};

pub fn draw(graphics: &mut dyn Graphics, index: &usize, frame: &Frame) {
    let translation: Matrix4<f32> = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [frame.position.x, frame.position.y, frame.position.z, 1.0],
    ]
    .into();

    let cos = frame.angle.cos();
    let sin = frame.angle.sin();
    let rotation: Matrix4<f32> = [
        [cos, sin, 0.0, 0.0],
        [-sin, cos, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
    .into();

    let mut transformation = translation * rotation;

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
        Model::Standing { skis: false } => skier_standing::WITHOUT_SKIS.iter(),
        Model::Standing { skis: true } => skier_standing::WITH_SKIS.iter(),
        Model::Sitting => skier_sitting::MODEL.iter(),
        Model::Chair => chair::MODEL.iter(),
    };
    let transformed_quads = quads
        .map(|quad| quad.transform(&transformation))
        .collect::<Vec<_>>();
    graphics.draw_quads(index, &transformed_quads).unwrap();
}
