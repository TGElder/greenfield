use commons::grid::Grid;

use crate::model::resource::RESOURCES;
use crate::model::source::Source;

pub fn run(population: &Grid<f32>, demand: &mut Grid<Vec<Source>>) {
    *demand = population.map(|tile, &population| {
        if population > 0.0 {
            let multiplier = population.floor() as usize;
            RESOURCES
                .into_iter()
                .flat_map(|resource| {
                    (0..multiplier).map(move |_| Source {
                        tile,
                        resource,
                        cost: 0,
                    })
                })
                .collect()
        } else {
            vec![]
        }
    });
}
