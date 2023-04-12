use commons::grid::Grid;
use commons::noise::simplex_noise;

use terrain_gen::heightmap_from_rises;
use terrain_gen::with_valleys::{heightmap_from_rises_with_valleys, ValleyParameters};

pub fn generate_heightmap() -> Grid<f32> {
    let power = 9;
    let weights = (0..power + 1)
        .map(|i| 1.0f32 / 1.125f32.powf((power - i) as f32))
        .collect::<Vec<_>>();
    let rises = simplex_noise(power, 1987, &weights)
        .normalize()
        .map(|_, z| (0.5 - z).abs() / 0.5);

    heightmap_from_rises(&rises, |xy| rises.is_border(xy)).map(|_, v| v * 128.0)
}
