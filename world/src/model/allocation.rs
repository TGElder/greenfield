use commons::geometry::XY;

use crate::model::resource::Resource;

pub struct Allocation {
    pub _from: XY<u32>,
    pub _from_market: XY<u32>,
    pub _to_market: XY<u32>,
    pub _resource: Resource,
}
