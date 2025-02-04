use commons::grid::Grid;

const TRAFFIC_THRESHOLD: usize = 4;
const MIN_DISTANCE: u64 = 25000;
const MAX_DISTANCE: u64 = 50000;

pub fn run(
    traffic: &Grid<usize>,
    distances: &Grid<u64>,
    towns: &mut Grid<bool>,
    population: &mut Grid<f32>,
) {
    let candidates = traffic
        .iter()
        .filter(|tile| !towns[tile])
        .filter(|tile| traffic[tile] >= TRAFFIC_THRESHOLD)
        .filter(|tile| distances[tile] >= MIN_DISTANCE)
        .filter(|tile| distances[tile] <= MAX_DISTANCE)
        .map(|xy| (xy, (traffic[xy], distances[xy])))
        .collect::<Vec<_>>();

    if let Some((tile, _)) = candidates.iter().max_by_key(|(_, score)| score) {
        towns[tile] = true;
        population[tile] = 1.0;
    }
}
