use std::collections::HashMap;

use commons::color::Rgb;
use engine::graphics::models::cube;
use engine::graphics::transform::Recolor;

use crate::draw::model::Model;

const COLOR: Rgb<f32> = Rgb::new(0.447, 0.361, 0.259);

pub fn base_station() -> Model<Rgb<f32>, ()> {
    Model {
        quads: cube::model().recolor(&|_| COLOR),
        attachment_points: HashMap::default(),
    }
}
