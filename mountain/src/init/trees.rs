use commons::grid::Grid;
use commons::noise::simplex_noise;

use crate::utils::ability::exposure;

pub fn generate_trees(power: u32, terrain: &Grid<f32>) -> Grid<bool> {
    let weights = (0..power + 1)
        .map(|i| 1.0f32 / 1.0f32.powf((power - i) as f32))
        .collect::<Vec<_>>();
    let noise = simplex_noise(power, 1990, &weights).normalize();

    noise.map(|position, value| {
        exposure(terrain, &position) <= 0.7
            && position.x % 4 == 0
            && position.y % 4 == 0
            && value.powf(2.0) * 1024.0 - 256.0 >= terrain[position]
    })
}
