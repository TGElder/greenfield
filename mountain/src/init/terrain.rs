use commons::grid::Grid;
use commons::noise::{simplex_noise, super_noise};

use terrain_gen::heightmap_from_rises;

pub fn generate_heightmap() -> Grid<f32> {
    let power = 10;
    let octaves = (0..power + 1)
        .map(|i| 1.0f32 / 2.0f32.powf((power - i) as f32))
        .collect::<Vec<_>>();
    let weights = (0..power + 1)
        .map(|i| 1.0f32 / 1.4f32.powf((power - i) as f32))
        .collect::<Vec<_>>();
    let weights = weights
        .iter()
        .enumerate()
        .map(|(i, weight)| {
            simplex_noise(power, (3504 + i + 1) as i32, &octaves)
                .normalize()
                .map(|_, z| z * weight)
        })
        .collect::<Vec<_>>();
    let rises = super_noise(power, 3504, &weights)
        .normalize()
        .map(|_, z| (0.5 - z).abs() / 0.5);

    heightmap_from_rises(&rises, |xy| xy.x == 0 && xy.y == 0).map(|_, v| v * 1.0)
}
