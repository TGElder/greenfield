use commons::color::Rgb;
use engine::graphics::elements::Triangle;
use engine::graphics::transform::Recolor;

use crate::draw::model::pyramid;

const GREEN: Rgb<f32> = Rgb::new(0.0, 0.373, 0.337);
const BROWN: Rgb<f32> = Rgb::new(0.447, 0.361, 0.259);

pub fn model() -> Vec<Triangle<Rgb<f32>>> {
    pyramid::model(0.025, -0.025, 1.0)
        .recolor(&|_| BROWN)
        .into_iter()
        .chain(pyramid::model(0.125, 0.5, 1.0).recolor(&|_| GREEN))
        .chain(pyramid::model(0.15, 0.2, 0.8).recolor(&|_| GREEN))
        .collect()
}
