use commons::color::Rgb;
use commons::geometry::xyz;
use commons::grid::Grid;
use engine::graphics::elements::Quad;
use engine::graphics::Graphics;
use nalgebra::Matrix4;

use crate::draw::transform_quad;
use crate::model::Avatar;

pub fn draw_avatar(
    avatar: &Avatar,
    terrain: &Grid<f32>,
    index: &usize,
    graphics: &mut dyn Graphics,
) {
    let Avatar::Static(state) = avatar else {return};

    let translation: Matrix4<f32> = [
        [1.0, 0.0, 0.0, state.position.x as f32],
        [0.0, 1.0, 0.0, state.position.y as f32],
        [0.0, 0.0, 1.0, terrain[state.position] * 32.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
    .into();
    let translation = translation.transpose();

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
                transform_quad(&SKIS, &transformation),
                transform_quad(&BODY_FRONT, &transformation),
                transform_quad(&BODY_BACK, &transformation),
            ],
        )
        .unwrap();

    //     graphics.draw_quads(index, &[
    //     SKIS,
    //     BODY_FRONT,
    //     BODY_BACK,
    // ]).unwrap();
}

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
