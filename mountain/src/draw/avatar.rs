use commons::color::Rgb;
use commons::geometry::{xyz, XYZ};

use engine::graphics::elements::Quad;
use engine::graphics::transform::Transform;
use engine::graphics::Graphics;
use nalgebra::Matrix4;

use crate::model::frame::{Frame, Mode};

const STANDING_SKIS: Quad = Quad {
    color: Rgb::new(1.0, 0.0, 0.0),
    corners: [
        xyz(-0.5, -0.25, 0.0),
        xyz(0.5, -0.25, 0.0),
        xyz(0.5, 0.25, 0.0),
        xyz(-0.5, 0.25, 0.0),
    ],
};

const STANDING_BODY_FRONT: Quad = Quad {
    color: Rgb::new(1.0, 0.0, 0.0),
    corners: [
        xyz(0.0, -0.25, 0.0),
        xyz(0.0, 0.25, 0.0),
        xyz(0.0, 0.25, 1.0),
        xyz(0.0, -0.25, 1.0),
    ],
};

const STANDING_BODY_BACK: Quad = Quad {
    color: Rgb::new(1.0, 0.0, 0.0),
    corners: [
        xyz(0.0, -0.25, 1.0),
        xyz(0.0, 0.25, 1.0),
        xyz(0.0, 0.25, 0.0),
        xyz(0.0, -0.25, 0.0),
    ],
};

const SITTING_TORSO_FRONT: Quad = Quad {
    color: Rgb::new(1.0, 0.0, 0.0),
    corners: [
        xyz(0.0, -0.25, 0.0),
        xyz(0.0, 0.25, 0.0),
        xyz(0.0, 0.25, 0.5),
        xyz(0.0, -0.25, 0.5),
    ],
};

const SITTING_TORSO_BACK: Quad = Quad {
    color: Rgb::new(1.0, 0.0, 0.0),
    corners: [
        xyz(0.0, -0.25, 0.5),
        xyz(0.0, 0.25, 0.5),
        xyz(0.0, 0.25, 0.0),
        xyz(0.0, -0.25, 0.0),
    ],
};

const SITTING_LEGS_TOP: Quad = Quad {
    color: Rgb::new(1.0, 0.0, 0.0),
    corners: [
        xyz(0.25, -0.25, 0.0),
        xyz(0.25, 0.25, 0.0),
        xyz(0.0, 0.25, 0.0),
        xyz(0.0, -0.25, 0.0),
    ],
};

const SITTING_LEGS_BOTTOM_FRONT: Quad = Quad {
    color: Rgb::new(1.0, 0.0, 0.0),
    corners: [
        xyz(0.25, -0.25, -0.25),
        xyz(0.25, 0.25, -0.25),
        xyz(0.25, 0.25, 0.0),
        xyz(0.25, -0.25, 0.0),
    ],
};

const SITTING_LEGS_BOTTOM_BACK: Quad = Quad {
    color: Rgb::new(1.0, 0.0, 0.0),
    corners: [
        xyz(0.25, -0.25, 0.0),
        xyz(0.25, 0.25, 0.0),
        xyz(0.25, 0.25, -0.25),
        xyz(0.25, -0.25, -0.25),
    ],
};

const SITTING_SKIS: Quad = Quad {
    color: Rgb::new(1.0, 0.0, 0.0),
    corners: [
        xyz(-0.25, -0.25, -0.25),
        xyz(0.75, -0.25, -0.25),
        xyz(0.75, 0.25, -0.25),
        xyz(-0.25, 0.25, -0.25),
    ],
};

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

    match frame.mode {
        Mode::Walking => {
            graphics
                .draw_quads(
                    index,
                    &[
                        STANDING_BODY_FRONT.transform(&transformation),
                        STANDING_BODY_BACK.transform(&transformation),
                    ],
                )
                .unwrap();
        }
        Mode::Skiing => {
            graphics
                .draw_quads(
                    index,
                    &[
                        STANDING_SKIS.transform(&transformation),
                        STANDING_BODY_FRONT.transform(&transformation),
                        STANDING_BODY_BACK.transform(&transformation),
                    ],
                )
                .unwrap();
        }
        Mode::Sitting => {
            graphics
                .draw_quads(
                    index,
                    &[
                        SITTING_TORSO_FRONT.transform(&transformation),
                        SITTING_TORSO_BACK.transform(&transformation),
                        SITTING_LEGS_TOP.transform(&transformation),
                        SITTING_LEGS_BOTTOM_FRONT.transform(&transformation),
                        SITTING_LEGS_BOTTOM_BACK.transform(&transformation),
                        SITTING_SKIS.transform(&transformation),
                    ],
                )
                .unwrap();
        }
    }
}
