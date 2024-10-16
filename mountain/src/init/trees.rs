use std::f32::consts::PI;

use commons::geometry::xy;
use commons::grid::Grid;
use commons::noise::simplex_noise;
use commons::scale::Scale;
use rand::{thread_rng, Rng};

use crate::model::ability::Ability;
use crate::model::tree::Tree;
use crate::utils::ability::exposure;

const STRIP_WIDTH: u32 = 4;
const BORDER_ELEVATION: f32 = 192.0; // Tree probability is 1.0 at elevation 0 but you probably don't want probability 1.0.
const SEA_LEVEL_MAX_TREE_HEIGHT: f32 = 40.0;
const TREE_LINE_MAX_TREE_HEIGHT: f32 = 1.0;

pub struct Parameters {
    pub power: u32,
    pub tree_line_elevation: f32,
}

pub fn generate_trees(
    terrain: &Grid<f32>,
    Parameters {
        power,
        tree_line_elevation,
    }: Parameters,
) -> Grid<Option<Tree>> {
    let weights = vec![1.0; power as usize];
    let noise = simplex_noise(power, 1990, &weights).normalize();

    let noise_to_max_elevation = Scale::new((0.0, 1.0), (0.0, tree_line_elevation));
    let noise_to_tree_height = Scale::new(
        (0.0, 1.0),
        (SEA_LEVEL_MAX_TREE_HEIGHT, TREE_LINE_MAX_TREE_HEIGHT),
    );

    let mut out = noise.map(|_, _| None);
    let mut rng = thread_rng();
    let tree_strip_count = 2u32.pow(power) / STRIP_WIDTH;
    for x_strip in 0..tree_strip_count {
        for y_strip in 0..tree_strip_count {
            let position = xy(
                random_value_in_strip(&mut rng, x_strip),
                random_value_in_strip(&mut rng, y_strip),
            );
            if exposure(terrain, &position) > Ability::Expert.max_exposure() {
                continue;
            }

            let noise = noise[position];
            let elevation = terrain[position] + BORDER_ELEVATION;
            let max_elevation = noise_to_max_elevation.scale(noise);
            if elevation > max_elevation {
                continue;
            }

            out[position] = Some(Tree {
                yaw: rng.gen::<f32>() * 2.0 * PI,
                height: noise_to_tree_height.scale(noise),
            })
        }
    }

    out
}

fn random_value_in_strip<R: Rng>(rng: &mut R, strip: u32) -> u32 {
    (strip * STRIP_WIDTH) + rng.gen_range(0..4)
}
