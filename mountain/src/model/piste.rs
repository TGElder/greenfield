use commons::origin_grid::OriginGrid;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Class {
    Piste,
    Path,
}

#[derive(Serialize, Deserialize)]
pub struct Piste {
    pub class: Class,
    pub grid: OriginGrid<bool>,
}
