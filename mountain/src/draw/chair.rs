use commons::color::Rgb;
use commons::geometry::{xyz, XYZ};

use engine::graphics::elements::Quad;
use engine::graphics::transform::Transform;
use engine::graphics::Graphics;
use nalgebra::Matrix4;

static BLACK: Rgb<f32> = Rgb::new(0.0, 0.0, 0.0);

static POLE_FRONT: Quad = Quad {
    color: BLACK,
    corners: [
        xyz(0.0, -0.05, -1.0),
        xyz(0.0, 0.05, -1.0),
        xyz(0.0, 0.05, -0.0),
        xyz(0.0, -0.05, -0.0),
    ],
};

static POLE_BACK: Quad = Quad {
    color: BLACK,
    corners: [
        xyz(0.0, -0.05, -0.0),
        xyz(0.0, 0.05, -0.0),
        xyz(0.0, 0.05, -1.0),
        xyz(0.0, -0.05, -1.0),
    ],
};

static CHAIR_REST_FRONT: Quad = Quad {
    color: BLACK,
    corners: [
        xyz(0.0, -0.75, -1.5),
        xyz(0.0, 0.75, -1.5),
        xyz(0.0, 0.75, -1.0),
        xyz(0.0, -0.75, -1.0),
    ],
};

static CHAIR_REST_BACK: Quad = Quad {
    color: BLACK,
    corners: [
        xyz(0.0, -0.75, -1.0),
        xyz(0.0, 0.75, -1.0),
        xyz(0.0, 0.75, -1.5),
        xyz(0.0, -0.75, -1.5),
    ],
};

static CHAIR_SEAT: Quad = Quad {
    color: BLACK,
    corners: [
        xyz(0.5, -0.75, -1.5),
        xyz(0.5, 0.75, -1.5),
        xyz(0.0, 0.75, -1.5),
        xyz(0.0, -0.75, -1.5),
    ],
};

pub fn draw(graphics: &mut dyn Graphics, index: &usize, position: &XYZ<f32>, angle: f32) {
    let translation: Matrix4<f32> = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [position.x, position.y, position.z, 1.0],
    ]
    .into();

    let cos = angle.cos();
    let sin = angle.sin();
    let rotation: Matrix4<f32> = [
        [cos, sin, 0.0, 0.0],
        [-sin, cos, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
    .into();

    let transformation = translation * rotation;

    graphics
        .draw_quads(
            index,
            &[
                POLE_FRONT.transform(&transformation),
                POLE_BACK.transform(&transformation),
                CHAIR_REST_FRONT.transform(&transformation),
                CHAIR_REST_BACK.transform(&transformation),
                CHAIR_SEAT.transform(&transformation),
            ],
        )
        .unwrap();
}
