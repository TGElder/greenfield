use std::collections::HashMap;

use commons::geometry::XYZ;
use engine::graphics::elements::Quad;

pub mod chair;
pub mod pyramid;
pub mod skier;
pub mod skier_sitting;
pub mod skier_standing;
pub mod tree;

pub struct Model<T> {
    pub quads: Vec<Quad>,
    pub attachment_points: HashMap<T, XYZ<f32>>,
}
