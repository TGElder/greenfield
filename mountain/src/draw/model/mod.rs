use std::collections::HashMap;

use commons::geometry::XYZ;
use engine::graphics::elements::Quad;

pub mod building;
pub mod chair;
pub mod lift_building;
pub mod prism;
pub mod pyramid;
pub mod skier;
pub mod skier_sitting;
pub mod skier_standing;
pub mod tree;
pub mod window;

pub struct Model<T, U> {
    pub quads: Vec<Quad<T>>,
    pub attachment_points: HashMap<U, XYZ<f32>>,
}
