use commons::color::Rgb;
use engine::graphics::elements::Triangle;

use crate::draw::model::pyramid;

const GREEN: Rgb<f32> = Rgb::new(0.0, 0.373, 0.337);
const BROWN: Rgb<f32> = Rgb::new(0.447, 0.361, 0.259);

pub fn model() -> Vec<Triangle> {
    pyramid::model(0.5, -0.5, 10.0, BROWN)
        .into_iter()
        .chain(pyramid::model(2.5, 10.0, 20.0, GREEN))
        .chain(pyramid::model(3.0, 4.0, 16.0, GREEN))
        .collect()
}
