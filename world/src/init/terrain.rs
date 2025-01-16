use commons::grid::Grid;
use commons::noise::simplex_noise;

use terrain_gen::with_valleys::{heightmap_from_rises_with_valleys, ValleyParameters};

#[derive(Clone)]
pub struct Parameters {
    pub power: u32,
    pub seed: i32,
}

pub fn generate_heightmap(Parameters { power, seed }: Parameters) -> Grid<f32> {
    let weights = (0..power + 1)
        .map(|i| 1.0f32 / 1.125f32.powf((power - i) as f32))
        .collect::<Vec<_>>();
    let rises = simplex_noise(power, seed, &weights)
        .normalize()
        .map(|_, z| (0.5 - z).abs() / 0.5);

    heightmap_from_rises_with_valleys(
        &rises,
        ValleyParameters {
            height_threshold: 0.25,
            rain_threshold: 256,
            rise: 0.01,
            origin_fn: |xy| rises.is_border(xy),
        },
    )
    .map(|_, z| z * 64.0)
}
