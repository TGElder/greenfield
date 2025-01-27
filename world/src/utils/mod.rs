use commons::geometry::XY;
use commons::grid::{Grid, CORNERS};

pub fn tile_heights(terrain: &Grid<f32>) -> Grid<f32> {
    Grid::from_fn(terrain.width() - 1, terrain.height() - 1, |xy| {
        terrain
            .offsets(xy, &CORNERS)
            .map(|corner| terrain[corner])
            .sum::<f32>()
            / 4.0
    })
}

pub fn is_cliff(position: &XY<u32>, tile_heights: &Grid<f32>, cliff_rise: f32) -> bool {
    tile_heights
        .neighbours_4(position)
        .any(|neighbour| (tile_heights[position] - tile_heights[neighbour]).abs() >= cliff_rise)
}

pub fn cost(
    from: &XY<u32>,
    to: &XY<u32>,
    tile_heights: &Grid<f32>,
    cliff_rise: f32,
) -> Option<u32> {
    let rise = (tile_heights[from] - tile_heights[to]).abs();
    if rise > cliff_rise {
        return None;
    }
    Some(((rise + 1.0) * 1000.0).round() as u32)
}
