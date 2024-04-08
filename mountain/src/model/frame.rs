use commons::color::Rgb;
use commons::geometry::XYZ;

use crate::model::skier::Clothes;

#[derive(Clone, Copy)]
pub struct Frame {
    pub position: XYZ<f32>,
    pub yaw: f32,
    pub pitch: f32,
    pub model_offset: Option<XYZ<f32>>,
    pub model: Model,
}

#[derive(Clone, Copy, Debug)]
pub enum Model {
    Standing {
        skis: bool,
        clothes: Clothes<Rgb<f32>>,
    },
    Sitting {
        clothes: Clothes<Rgb<f32>>,
    },
    Chair,
}
