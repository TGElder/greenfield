use commons::geometry::{xyz, XYZ};

use crate::graphics::elements::Quad;

const BASE: [XYZ<f32>; 4] = [
    xyz(-0.5, -0.5, -0.5),
    xyz(-0.5, 0.5, -0.5),
    xyz(0.5, 0.5, -0.5),
    xyz(0.5, -0.5, -0.5),
];

const TOP: [XYZ<f32>; 4] = [
    xyz(-0.5, -0.5, 0.5),
    xyz(0.5, -0.5, 0.5),
    xyz(0.5, 0.5, 0.5),
    xyz(-0.5, 0.5, 0.5),
];

pub fn model() -> Vec<Quad<Side>> {
    vec![
        Quad {
            corners: [TOP[3], BASE[1], BASE[0], TOP[0]],
            color: Side::Left,
        },
        Quad {
            corners: [TOP[1], BASE[3], BASE[2], TOP[2]],
            color: Side::Right,
        },
        Quad {
            corners: [TOP[0], BASE[0], BASE[3], TOP[1]],
            color: Side::Back,
        },
        Quad {
            corners: [TOP[2], BASE[2], BASE[1], TOP[3]],
            color: Side::Front,
        },
        Quad {
            corners: BASE,
            color: Side::Bottom,
        },
        Quad {
            corners: TOP,
            color: Side::Top,
        },
    ]
}

#[derive(Clone, Copy, PartialEq)]
pub enum Side {
    Right,
    Left,
    Back,
    Front,
    Bottom,
    Top,
}

impl Side {
    pub fn index(&self) -> usize {
        match self {
            Side::Right => 0,
            Side::Left => 1,
            Side::Back => 2,
            Side::Front => 3,
            Side::Bottom => 4,
            Side::Top => 5,
        }
    }
}
