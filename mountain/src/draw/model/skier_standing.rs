use std::f32::consts::PI;

use commons::geometry::xyz;

use crate::draw::model::{skier, Model};

pub fn model() -> Model<skier::Color, skier::AttachmentPoints> {
    skier::model(skier::Parameters {
        lower_leg_pitch: PI / 16.0,
        lower_leg_scale: xyz(0.2, 0.4, 0.5),
        upper_leg_pitch: -PI / 4.0,
        upper_leg_scale: xyz(0.2, 0.4, 0.4),
        torso_pitch: PI / 6.0,
        torso_scale: xyz(0.2, 0.5, 0.55),
        head_pitch: 0.0,
        head_scale: xyz(0.25, 0.25, 0.25),
    })
}
