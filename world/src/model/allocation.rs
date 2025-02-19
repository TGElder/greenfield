use commons::geometry::XY;

use crate::model::resource::Resource;

pub struct Allocation {
    pub from: XY<u32>,
    pub from_market: XY<u32>,
    pub to_market: XY<u32>,
    pub resource: Resource,
}
