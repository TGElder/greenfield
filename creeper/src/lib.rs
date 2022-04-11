use commons::grid::Grid;

pub fn generate_heightmap(rises: Grid<f32>) -> Grid<f32> {
    let stack = vec![];
    Grid::default(rises.width(), rises.height())
    
}

struct StackElement{
    height: f32,
    position: (u32, u32)
}


#[cfg(test)]
mod tests {
    use commons::noise::simplex_noise;

    use crate::generate_heightmap;

    #[test]
    fn test() {
        let weights = (0..8).map(|i| 2.0f32.powf(i as f32)).collect::<Vec<_>>();
        let rises = simplex_noise(8, 1986, &weights);
        let heightmap = generate_heightmap(rises);
        heightmap.to_image("test_resources/test.png");
    }
}
