mod downhills;
mod heightmap_from_rises;
mod heightmap_from_rises_with_valleys;
mod rain;

use commons::grid::Grid;
pub use downhills::*;
pub use heightmap_from_rises::*;
pub use heightmap_from_rises_with_valleys::*;
pub use rain::*;

pub type Heightmap = Grid<f32>;
pub type Rises = Grid<f32>;
