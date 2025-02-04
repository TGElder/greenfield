use std::collections::HashMap;

use commons::color::Rgb;
use commons::geometry::xyz;
use engine::graphics::elements::Quad;

use crate::draw::model::Model;

const COLOR: Rgb<f32> = Rgb::new(0.389, 0.389, 0.389);

const POLE_FRONT: Quad<Rgb<f32>> = Quad {
    color: COLOR,
    corners: [
        xyz(0.0, -0.05, -1.0),
        xyz(0.0, 0.05, -1.0),
        xyz(0.0, 0.05, -0.0),
        xyz(0.0, -0.05, -0.0),
    ],
};

const POLE_BACK: Quad<Rgb<f32>> = Quad {
    color: COLOR,
    corners: [
        xyz(0.0, -0.05, -0.0),
        xyz(0.0, 0.05, -0.0),
        xyz(0.0, 0.05, -1.0),
        xyz(0.0, -0.05, -1.0),
    ],
};

const CHAIR_REST_FRONT: Quad<Rgb<f32>> = Quad {
    color: COLOR,
    corners: [
        xyz(0.0, -0.75, -1.5),
        xyz(0.0, 0.75, -1.5),
        xyz(0.0, 0.75, -1.0),
        xyz(0.0, -0.75, -1.0),
    ],
};

const CHAIR_REST_BACK: Quad<Rgb<f32>> = Quad {
    color: COLOR,
    corners: [
        xyz(0.0, -0.75, -1.0),
        xyz(0.0, 0.75, -1.0),
        xyz(0.0, 0.75, -1.5),
        xyz(0.0, -0.75, -1.5),
    ],
};

const CHAIR_SEAT: Quad<Rgb<f32>> = Quad {
    color: COLOR,
    corners: [
        xyz(0.5, -0.75, -1.5),
        xyz(0.5, 0.75, -1.5),
        xyz(0.0, 0.75, -1.5),
        xyz(0.0, -0.75, -1.5),
    ],
};

#[derive(Eq, Hash, PartialEq)]
pub enum AttachmentPoints {
    FrontOfChair,
}

pub fn model() -> Model<Rgb<f32>, AttachmentPoints> {
    Model {
        quads: vec![
            POLE_FRONT,
            POLE_BACK,
            CHAIR_REST_FRONT,
            CHAIR_REST_BACK,
            CHAIR_SEAT,
        ],
        attachment_points: HashMap::from_iter([(
            AttachmentPoints::FrontOfChair,
            (CHAIR_SEAT.corners[0] + CHAIR_SEAT.corners[1]) / 2.0,
        )]),
    }
}
