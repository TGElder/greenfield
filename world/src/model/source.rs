use commons::geometry::XY;

use crate::model::resource::Resource;

pub struct Source {
    pub tile: XY<u32>,
    pub resource: Resource,
    pub cost: u64,
}
