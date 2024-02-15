use commons::grid::Grid;
use commons::noise::simplex_noise;
use commons::scale::Scale;

use crate::model::ability::Ability;
use crate::utils::ability::exposure;

pub fn generate_trees(power: u32, terrain: &Grid<f32>) -> Grid<bool> {
    let weights = vec![1.0; power as usize];
    let noise = simplex_noise(power, 1990, &weights).normalize();

    let min_elevation = 192.0; // elevation at border. Tree probability is 1.0 at elevation 0 but you probably don't want this.
    let tree_line_elevation = 512.0;
    let scale = Scale::new(
        (0.0, 1.0),
        (-min_elevation, tree_line_elevation - min_elevation),
    );

    noise.map(|position, value| {
        exposure(terrain, &position) <= Ability::Expert.max_exposure()
            && position.x % 4 == 0
            && position.y % 4 == 0
            && scale.scale(value) >= terrain[position]
    })
}
