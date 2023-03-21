use commons::color::Rgb;
use commons::geometry::xyz;

use engine::graphics::elements::Quad;
use engine::graphics::transform::Transform;
use engine::graphics::Graphics;
use nalgebra::Matrix4;

use crate::model::Avatar;

static SKIS: Quad = Quad {
    color: Rgb::new(1.0, 0.0, 0.0),
    corners: [
        xyz(-0.25, -0.5, 0.0),
        xyz(0.25, -0.5, 0.0),
        xyz(0.25, 0.5, 0.0),
        xyz(-0.25, 0.5, 0.0),
    ],
};

static BODY_FRONT: Quad = Quad {
    color: Rgb::new(1.0, 0.0, 0.0),
    corners: [
        xyz(-0.25, 0.0, 0.0),
        xyz(0.25, 0.0, 0.0),
        xyz(0.25, 0.0, 1.0),
        xyz(-0.25, 0.0, 1.0),
    ],
};

static BODY_BACK: Quad = Quad {
    color: Rgb::new(1.0, 0.0, 0.0),
    corners: [
        xyz(-0.25, 0.0, 1.0),
        xyz(0.25, 0.0, 1.0),
        xyz(0.25, 0.0, 0.0),
        xyz(-0.25, 0.0, 0.0),
    ],
};

pub fn draw_avatar(avatar: &Avatar, micros: &u64, graphics: &mut dyn Graphics, index: &usize) {
    let Some(state) = avatar.state(micros) else {return};

    let translation: Matrix4<f32> = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [
            state.position.x,
            state.position.y,
            state.position.z * 32.0,
            1.0,
        ],
    ]
    .into();

    let cos = state.angle.cos();
    let sin = state.angle.sin();
    let rotation: Matrix4<f32> = [
        [cos, -sin, 0.0, 0.0],
        [sin, cos, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
    .into();

    let transformation = translation * rotation;

    graphics
        .draw_quads(
            index,
            &[
                SKIS.transform(&transformation),
                BODY_FRONT.transform(&transformation),
                BODY_BACK.transform(&transformation),
            ],
        )
        .unwrap();
}
