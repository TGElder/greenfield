use commons::color::Rgb;
use engine::graphics::elements::Triangle;
use engine::graphics::transform::Recolor;

use crate::draw::model::pyramid;

const LEAF_COLOR: Rgb<f32> = Rgb::new(0.0, 0.639, 0.61);
const TRUNK_COLOR: Rgb<f32> = Rgb::new(0.694, 0.629, 0.541);

pub fn model() -> Vec<Triangle<Rgb<f32>>> {
    pyramid::model(0.025, -0.025, 1.0)
        .recolor(&|_| TRUNK_COLOR)
        .into_iter()
        .chain(pyramid::model(0.125, 0.5, 1.0).recolor(&|_| LEAF_COLOR))
        .chain(pyramid::model(0.15, 0.2, 0.8).recolor(&|_| LEAF_COLOR))
        .collect()
}
