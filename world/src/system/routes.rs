use std::collections::HashMap;

use commons::geometry::XY;
use network::algorithms::floyd_warshall::floyd_warshall;
use network::model::Edge;

use crate::model::path::Path;

pub fn run(
    paths: &HashMap<(XY<u32>, XY<u32>), Path>,
    routes: &mut HashMap<(XY<u32>, XY<u32>), Path>,
) {
    let edges = paths
        .iter()
        .map(|((from, to), path)| Edge {
            from,
            to,
            cost: path.cost as u32, // TODO
        })
        .collect::<Vec<_>>();

    let result = floyd_warshall(&edges);

    for ((from, to), result) in result {
        routes.insert(
            (*from, *to),
            Path {
                cost: result.cost,
                _tiles: result.path.into_iter().copied().collect(),
            },
        );
    }
}
