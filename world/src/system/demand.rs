use commons::grid::Grid;

use crate::model::resource::RESOURCES;
use crate::model::source::Source;

pub fn run(towns: &Grid<bool>, demand: &mut Grid<Vec<Source>>) {
    *demand = towns.map(|tile, &is_town| {
        if is_town {
            RESOURCES
                .into_iter()
                .map(|resource| Source {
                    tile,
                    resource,
                    cost: 0,
                })
                .collect()
        } else {
            vec![]
        }
    });
}
