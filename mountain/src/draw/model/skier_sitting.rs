use std::collections::HashMap;
use std::f32::consts::PI;

use commons::geometry::xyz;
use engine::graphics::transform::Transform;
use engine::graphics::utils::translation_matrix;

use crate::draw::model::{chair, skier, Model};

pub fn model() -> Model<()> {
    let skier_parameters = skier::Parameters {
        lower_leg_pitch: 0.0,
        lower_leg_scale: xyz(0.2, 0.5, 0.5),
        upper_leg_pitch: -PI / 2.0,
        upper_leg_scale: xyz(0.2, 0.5, 0.4),
        torso_pitch: 0.0,
        torso_scale: xyz(0.2, 0.5, 0.55),
        head_pitch: 0.0,
        head_scale: xyz(0.25, 0.25, 0.25),
    };
    let heels_to_knee = -xyz(
        0.0,
        0.0,
        skier_parameters.lower_leg_scale.z - skier_parameters.upper_leg_scale.x,
    );
    let skier = skier::model(skier_parameters);
    let chair = chair::model();
    let offset = chair.attachment_points[&chair::AttachmentPoints::FrontOfChair]
        - skier.attachment_points[&skier::AttachmentPoints::BackOfHeels]
        + heels_to_knee;
    Model {
        quads: skier.quads.transform(&translation_matrix(offset)),
        attachment_points: HashMap::new(),
    }
}
