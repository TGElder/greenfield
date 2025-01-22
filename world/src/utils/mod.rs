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
