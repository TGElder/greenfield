use std::collections::HashMap;

use commons::grid::Grid;
use commons::noise::simplex_noise;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::model::resource::{Resource, RESOURCES};

pub fn generate_resources(
    power: u32,
    tile_heights: &Grid<f32>,
    sea_level: f32,
) -> Grid<Option<Resource>> {
    let weights = (0..power + 1)
        .map(|i| 1.0f32 / 1.5f32.powf((power - i) as f32))
        .collect::<Vec<_>>();
    let values = simplex_noise(power, 1, &weights).normalize();

    let mut resources = RESOURCES;
    resources.shuffle(&mut thread_rng());

    let mut counts: HashMap<Resource, usize> = HashMap::new();

    let out = tile_heights.map(|xy, &z| {
        if z <= sea_level {
            return None;
        }
        for (i, resource) in resources.iter().enumerate() {
            let from = 0.2 + (0.0625 * 0.6 * (i as f32));
            let to = 0.2 + (0.0625 * 0.6 * (i as f32 + 0.05));
            if values[xy] >= from && values[xy] < to {
                *counts.entry(*resource).or_default() += 1;
                return Some(*resource);
            }
        }
        None
    });

    println!("{:?}", counts);

    out
}
