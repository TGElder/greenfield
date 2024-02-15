use commons::grid::Grid;
use commons::noise::simplex_noise;
use commons::scale::Scale;
use rand::random;

use crate::model::ability::Ability;
use crate::model::tree::Tree;
use crate::utils::ability::exposure;

pub fn generate_trees(power: u32, terrain: &Grid<f32>) -> Grid<Option<Tree>> {
    let weights = vec![1.0; power as usize];
    let noise = simplex_noise(power, 1990, &weights).normalize();

    let min_elevation = 192.0; // elevation at border. Tree probability is 1.0 at elevation 0 but you probably don't want probability 1.0.
    let tree_line_elevation = 512.0;
    let scale = Scale::new(
        (0.0, 1.0),
        (-min_elevation, tree_line_elevation - min_elevation),
    );

    let spacing = 4;
    noise.map(|position, value| {
        if exposure(terrain, &position) > Ability::Expert.max_exposure()
            || position.x % spacing != 0
            || position.y % spacing != 0
            || scale.scale(value) < terrain[position]
        {
            return None;
        }
        Some(Tree { yaw: random() })
    })
}
