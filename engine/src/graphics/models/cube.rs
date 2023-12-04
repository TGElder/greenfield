use commons::color::Rgb;
use commons::geometry::{xyz, XYZ};

use crate::graphics::elements::Quad;

const BASE: [XYZ<f32>; 4] = [
    xyz(-0.5, -0.5, -0.5), //LA
    xyz(-0.5, 0.5, -0.5),  //LD
    xyz(0.5, 0.5, -0.5),   //LC
    xyz(0.5, -0.5, -0.5),  //LB
];

const TOP: [XYZ<f32>; 4] = [
    xyz(-0.5, -0.5, 0.5),
    xyz(0.5, -0.5, 0.5),
    xyz(0.5, 0.5, 0.5),
    xyz(-0.5, 0.5, 0.5),
];

pub const fn model(color: Rgb<f32>) -> [Quad; 6] {
    [
        Quad {
            corners: BASE,
            color,
        },
        Quad {
            corners: TOP,
            color,
        },
        Quad {
            corners: [TOP[0], BASE[0], BASE[3], TOP[1]],
            color,
        },
        Quad {
            corners: [TOP[2], BASE[2], BASE[1], TOP[3]],
            color,
        },
        Quad {
            corners: [TOP[1], BASE[3], BASE[2], TOP[2]],
            color,
        },
        Quad {
            corners: [TOP[3], BASE[1], BASE[0], TOP[0]],
            color,
        },
    ]
}
