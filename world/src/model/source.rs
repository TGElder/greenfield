use commons::geometry::XY;

use crate::model::resource::Resource;

pub struct Source {
    pub _tile: XY<u32>,
    pub _resource: Resource,
}
