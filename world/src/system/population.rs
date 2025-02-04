use commons::grid::Grid;

pub fn run(population: &mut Grid<f32>) {
    *population = population.map(|_, value| value * 1.05);
    for tile in population.iter() {
        if population[tile] > 0.0 {
            println!("{}={}", tile, population[tile]);
        }
    }
}
