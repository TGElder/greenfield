use commons::geometry::XY;
use commons::origin_grid::OriginGrid;

pub struct Selection {
    pub cells: Vec<XY<u32>>,
    pub grid: Option<OriginGrid<bool>>,
}

impl Default for Selection {
    fn default() -> Self {
        Self {
            cells: Vec::with_capacity(3),
            grid: None,
        }
    }
}
