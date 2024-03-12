use std::collections::HashMap;
use std::iter::once;

use commons::color::Rgb;
use commons::geometry::{xyz, XYZ};
use engine::graphics::elements::Quad;
use engine::graphics::models::cube::{self, Side};
use engine::graphics::transform::Transform;
use engine::graphics::utils::{transformation_matrix, translation_matrix};

use crate::draw::model::Model;

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

#[derive(Eq, Hash, PartialEq)]
pub enum AttachmentPoints {
    BackOfHeels,
}

pub struct Parameters {
    pub lower_leg_pitch: f32,
    pub lower_leg_scale: XYZ<f32>,
    pub upper_leg_pitch: f32,
    pub upper_leg_scale: XYZ<f32>,
    pub torso_pitch: f32,
    pub torso_scale: XYZ<f32>,
    pub head_pitch: f32,
    pub head_scale: XYZ<f32>,
}

pub fn model(
    Parameters {
        lower_leg_pitch,
        lower_leg_scale,
        upper_leg_pitch,
        upper_leg_scale,
        torso_pitch,
        torso_scale,
        head_pitch,
        head_scale,
    }: Parameters,
) -> Model<AttachmentPoints> {
    let lower_legs = cube::model(&|_| COLOR)
        .to_vec()
        .transform(&transformation_matrix(
            xyz(0.0, 0.0, 0.0),
            0.0,
            lower_leg_pitch,
            0.0,
            lower_leg_scale,
        ));
    let ski_center = (SKIS.corners[0] + SKIS.corners[1]) / 2.0;
    let offset = ski_center - lower_legs[Side::Bottom.index()].corners[3];
    let lower_legs = lower_legs.transform(&translation_matrix(offset));

    let upper_legs = cube::model(&|_| COLOR)
        .to_vec()
        .transform(&transformation_matrix(
            xyz(0.0, 0.0, 0.0),
            0.0,
            upper_leg_pitch,
            0.0,
            upper_leg_scale,
        ));
    let offset =
        lower_legs[Side::Top.index()].corners[2] - upper_legs[Side::Bottom.index()].corners[2];
    let upper_legs = upper_legs.transform(&translation_matrix(offset));
    let lower_legs_top = lower_legs[Side::Bottom.index()].corners;
    let back_of_heels = (lower_legs_top[0] + lower_legs_top[1]) / 2.0;

    let torso = cube::model(&|_| COLOR)
        .to_vec()
        .transform(&transformation_matrix(
            xyz(0.0, 0.0, 0.0),
            0.0,
            torso_pitch,
            0.0,
            torso_scale,
        ));
    let offset = upper_legs[Side::Top.index()].corners[0] - torso[Side::Bottom.index()].corners[0];
    let torso = torso.transform(&translation_matrix(offset));

    let head = cube::model(&|_| COLOR)
        .to_vec()
        .transform(&transformation_matrix(
            xyz(0.0, 0.0, 0.0),
            0.0,
            head_pitch,
            0.0,
            head_scale,
        ));
    let torso_center =
        (torso[Side::Top.index()].corners[0] + torso[Side::Top.index()].corners[3]) / 2.0;
    let head_center =
        (head[Side::Bottom.index()].corners[0] + head[Side::Bottom.index()].corners[1]) / 2.0;
    let offset = torso_center - head_center;
    let head = head.transform(&transformation_matrix(
        offset,
        0.0,
        0.0,
        0.0,
        xyz(1.0, 1.0, 1.0),
    ));

    let quads = once(SKIS)
        .chain(lower_legs)
        .chain(upper_legs)
        .chain(torso)
        .chain(head)
        .collect();

    Model {
        quads,
        attachment_points: HashMap::from_iter([(AttachmentPoints::BackOfHeels, back_of_heels)]),
    }
}
