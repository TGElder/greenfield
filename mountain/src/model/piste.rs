use commons::origin_grid::OriginGrid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Piste {
    pub grid: OriginGrid<bool>,
}
