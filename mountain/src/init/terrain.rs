use commons::grid::Grid;
use commons::noise::simplex_noise;

use commons::scale::Scale;
use terrain_gen::heightmap_from_rises;

pub fn generate_heightmap() -> Grid<f32> {
    let power = 11;
    let seed = 2001;
    let weights = (0..power + 1)
        .map(|i| if i > 10 {0.0}else{1.0f32 / 1.4f32.powf((power - i) as f32)})
        // .map(|i| if i == power {0.0}else{1.0f32 / 1.4f32.powf((power - i) as f32)})
        .collect::<Vec<_>>();
    let rises = simplex_noise(power, seed, &weights)
        .normalize()
        .map(|_, z| (0.5 - z).abs() / 0.5);

    let minimum_weights = (0..power + 1)
        .map(|i| 1.0f32 / 2.0f32.powf((power - i) as f32))
        .collect::<Vec<_>>();
    let minimums = simplex_noise(power, seed + 1, &minimum_weights)
        .normalize()
        .map(|_, z| (0.5 - z).abs() / 0.5);

    let rises = rises.map(|xy, z| Scale::new((0.0, 1.0), (0.05 + minimums[xy] * 0.15, 1.0)).scale(z));

    heightmap_from_rises(&rises, |xy| xy.x == 0 || xy.y == 0)
}
