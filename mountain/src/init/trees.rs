use commons::grid::Grid;
use commons::noise::simplex_noise;

pub fn generate_trees(power: u32, terrain: &Grid<f32>) -> Grid<bool> {
    let weights = (0..power + 1)
        .map(|i| 1.0f32 / 1.4f32.powf((power - i) as f32))
        .collect::<Vec<_>>();
    let noise = simplex_noise(power, 1989, &weights);

    noise.map(|position, value| *value <= terrain[position])
}
