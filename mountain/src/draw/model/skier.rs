use std::collections::HashMap;

use commons::geometry::{xyz, XYZ};
use engine::graphics::elements::Quad;
use engine::graphics::models::cube::{self, Side};
use engine::graphics::transform::{Recolor, Transform};
use engine::graphics::utils::{transformation_matrix, Transformation};

use crate::draw::model::Model;

const SKI_LEFT: Quad<Color> = Quad {
    color: Color::Skis,
    corners: [
        xyz(-0.8, -0.2, 0.0),
        xyz(0.8, -0.2, 0.0),
        xyz(0.8, -0.075, 0.0),
        xyz(-0.8, -0.075, 0.0),
    ],
};
const SKI_RIGHT: Quad<Color> = Quad {
    color: Color::Skis,
    corners: [
        xyz(-0.8, 0.075, 0.0),
        xyz(0.8, 0.075, 0.0),
        xyz(0.8, 0.2, 0.0),
        xyz(-0.8, 0.2, 0.0),
    ],
};

#[derive(Clone, Copy)]
pub enum Color {
    Skis,
    Trousers,
    Jacket,
    Helmet,
}

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
) -> Model<Color, AttachmentPoints> {
    let lower_legs = cube::model()
        .recolor(&|_| Color::Trousers)
        .transform(&transformation_matrix(Transformation {
            pitch: Some(lower_leg_pitch),
            scale: Some(lower_leg_scale),
            ..Transformation::default()
        }));
    let ski_center = (SKI_LEFT.corners[0] + SKI_RIGHT.corners[2]) / 2.0;
    let lower_legs_bottom = lower_legs[Side::Bottom.index()].corners;
    let back_of_heels = (lower_legs_bottom[0] + lower_legs_bottom[1]) / 2.0;
    let front_of_feet = (lower_legs_bottom[2] + lower_legs_bottom[3]) / 2.0;
    let offset = ski_center - front_of_feet;
    let lower_legs = lower_legs.transform(&transformation_matrix(Transformation {
        translation: Some(offset),
        ..Transformation::default()
    }));

    let upper_legs = cube::model()
        .recolor(&|_| Color::Trousers)
        .transform(&transformation_matrix(Transformation {
            pitch: Some(upper_leg_pitch),
            scale: Some(upper_leg_scale),
            ..Transformation::default()
        }));
    let offset =
        lower_legs[Side::Top.index()].corners[2] - upper_legs[Side::Bottom.index()].corners[2];
    let upper_legs = upper_legs.transform(&transformation_matrix(Transformation {
        translation: Some(offset),
        ..Transformation::default()
    }));

    let torso = cube::model()
        .recolor(&|_| Color::Jacket)
        .transform(&transformation_matrix(Transformation {
            pitch: Some(torso_pitch),
            scale: Some(torso_scale),
            ..Transformation::default()
        }));
    let top_of_upper_legs = upper_legs[Side::Top.index()].corners;
    let top_of_upper_legs_back = (top_of_upper_legs[0] + top_of_upper_legs[3]) / 2.0;
    let bottom_of_torso = torso[Side::Bottom.index()].corners;
    let bottom_of_torso_back = (bottom_of_torso[0] + bottom_of_torso[1]) / 2.0;
    let offset = top_of_upper_legs_back - bottom_of_torso_back;
    let torso = torso.transform(&transformation_matrix(Transformation {
        translation: Some(offset),
        ..Transformation::default()
    }));

    let head = cube::model()
        .recolor(&|_| Color::Helmet)
        .transform(&transformation_matrix(Transformation {
            pitch: Some(head_pitch),
            scale: Some(head_scale),
            ..Transformation::default()
        }));
    let torso_center =
        (torso[Side::Top.index()].corners[0] + torso[Side::Top.index()].corners[3]) / 2.0;
    let head_center =
        (head[Side::Bottom.index()].corners[0] + head[Side::Bottom.index()].corners[1]) / 2.0;
    let offset = torso_center - head_center;
    let head = head.transform(&transformation_matrix(Transformation {
        translation: Some(offset),
        ..Transformation::default()
    }));

    let quads = [SKI_LEFT, SKI_RIGHT]
        .into_iter()
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
