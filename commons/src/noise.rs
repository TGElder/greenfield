use simdnoise::NoiseBuilder;

use crate::grid::Grid;

pub fn simplex_noise(power: u32, seed: i32, octave_weights: &[f32]) -> Grid<f32> {
    let size_u32 = 2u32.pow(power);
    let size_usize = size_u32.try_into().unwrap();
    let noise_vector = octave_weights
        .iter()
        .enumerate()
        .map(|(i, weight)| {
            NoiseBuilder::gradient_2d(size_usize, size_usize)
                .with_seed(seed + i32::try_from(i).unwrap())
                .with_freq(1.0 / 2.0f32.powf(usize_to_f32(i)))
                .generate_scaled(0.0, *weight)
        })
        .fold(vec![0.0; size_usize * size_usize], sum_vectors);
    Grid::from_vec(size_u32, size_u32, noise_vector)
}

fn usize_to_f32(value: usize) -> f32 {
    u16::try_from(value).unwrap().try_into().unwrap()
}

fn sum_vectors(a: Vec<f32>, b: Vec<f32>) -> Vec<f32> {
    a.into_iter()
        .enumerate()
        .map(|(i, value)| value + b[i])
        .collect()
}

#[cfg(test)]
mod tests {
    use std::env::temp_dir;

    use super::*;

    #[test]
    fn test() {
        // given
        let weights = (0..8).map(|i| 2.0f32.powf(i as f32)).collect::<Vec<_>>();
        let noise = simplex_noise(8, 1986, &weights);

        // when
        let temp_path = temp_dir().join("test.png");
        let temp_path = temp_path.to_str().unwrap();
        noise.to_image(temp_path);

        // then
        let actual = image::open(temp_path).unwrap();
        let expected = image::open("test_resources/noise/test.png").unwrap();
        assert_eq!(actual, expected);
    }
}
