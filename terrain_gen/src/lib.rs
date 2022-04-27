mod downhills;
mod rain;
mod rises;
mod with_valleys;

use commons::grid::Grid;
pub use downhills::*;
pub use rain::*;
pub use rises::*;
pub use with_valleys::*;

pub type Heightmap = Grid<f32>;
