use commons::grid::Grid;
use commons::noise::simplex_noise;

use commons::scale::Scale;
use terrain_gen::heightmap_from_rises;

pub fn generate_heightmap(power: u32) -> Grid<f32> {
    let weights = (0..power + 1)
        .map(|i| 1.0f32 / 1.4f32.powf((power - i) as f32))
        .collect::<Vec<_>>();
    let rises = simplex_noise(power, 1989, &weights)
        .normalize()
        .map(|_, z| (0.5 - z).abs() / 0.5);

    let minimum_slope = 0.08;
    let maximum_slope = 1.0;
    let scale = Scale::new((0.0, 1.0), (minimum_slope, maximum_slope));
    let rises = rises.map(|_, z| scale.scale(z));

    heightmap_from_rises(&rises, |xy| xy.x == 0)
}
