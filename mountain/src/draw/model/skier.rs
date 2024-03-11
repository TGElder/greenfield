use std::iter::once;

use commons::color::Rgb;
use commons::geometry::xyz;
use engine::graphics::elements::Quad;
use engine::graphics::models::cube::{self, Side};
use engine::graphics::transform::Transform;
use engine::graphics::utils::{transformation_matrix, translation_matrix};

const COLOR: Rgb<f32> = Rgb::new(0.86, 0.01, 0.01);

const SKIS: Quad = Quad {
    color: COLOR,
    corners: [
        xyz(-0.8, -0.25, 0.0),
        xyz(0.8, -0.25, 0.0),
        xyz(0.8, 0.25, 0.0),
        xyz(-0.8, 0.25, 0.0),
    ],
};

pub fn model(lower_leg_pitch: f32, upper_leg_pitch: f32, torso_pitch: f32) -> Vec<Quad> {
    let lower_legs = cube::model(&|_| COLOR)
        .to_vec()
        .transform(&transformation_matrix(
            xyz(0.0, 0.0, 0.0),
            0.0,
            lower_leg_pitch,
            0.0,
            xyz(0.2, 0.5, 0.5),
        ));
    let ski_centre = (SKIS.corners[0] + SKIS.corners[1]) / 2.0;
    let offset = ski_centre - lower_legs[Side::Bottom.index()].corners[3];
    let lower_legs = lower_legs.transform(&translation_matrix(offset));

    let upper_legs = cube::model(&|_| COLOR)
        .to_vec()
        .transform(&transformation_matrix(
            xyz(0.0, 0.0, 0.0),
            0.0,
            upper_leg_pitch,
            0.0,
            xyz(0.2, 0.5, 0.4),
        ));
    let offset =
        lower_legs[Side::Top.index()].corners[2] - upper_legs[Side::Bottom.index()].corners[2];
    let upper_legs = upper_legs.transform(&translation_matrix(offset));

    let torso = cube::model(&|_| COLOR)
        .to_vec()
        .transform(&transformation_matrix(
            xyz(0.0, 0.0, 0.0),
            0.0,
            torso_pitch,
            0.0,
            xyz(0.2, 0.5, 0.55),
        ));
    let offset = upper_legs[Side::Top.index()].corners[0] - torso[Side::Bottom.index()].corners[0];
    let torso = torso.transform(&translation_matrix(offset));

    let head = cube::model(&|_| COLOR)
        .to_vec()
        .transform(&transformation_matrix(
            xyz(0.0, 0.0, 0.0),
            0.0,
            0.0,
            0.0,
            xyz(0.25, 0.25, 0.25),
        ));
    let torso_middle =
        (torso[Side::Top.index()].corners[0] + torso[Side::Top.index()].corners[3]) / 2.0;
    let head_middle =
        (head[Side::Bottom.index()].corners[0] + head[Side::Bottom.index()].corners[1]) / 2.0;
    let offset = torso_middle - head_middle;
    let head = head.transform(&transformation_matrix(
        offset,
        0.0,
        0.0,
        0.0,
        xyz(1.0, 1.0, 1.0),
    ));

    once(SKIS)
        .chain(lower_legs)
        .chain(upper_legs)
        .chain(torso)
        .chain(head)
        .collect()
}
