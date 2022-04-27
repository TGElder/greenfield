use std::borrow::Borrow;

use commons::grid::Grid;

use crate::{rises_to_heightmap, Downhill, Rain};

pub struct ValleyParameters {
    rain_threshold: usize,
    rise: f32,
}

pub trait WithValleys {
    fn with_valleys<B>(&self, parameters: B) -> Grid<f32>
    where
        B: Borrow<ValleyParameters>;
}

impl WithValleys for Grid<f32> {
    fn with_valleys<B>(&self, parameters: B) -> Grid<f32>
    where
        B: Borrow<ValleyParameters>,
    {
        let parameters = parameters.borrow();

        let rain = self.downhills().rain();

        let rises = self.map(|xy, z| {
            if rain[xy] > parameters.rain_threshold {
                parameters.rise
            } else {
                *z
            }
        });

        rises_to_heightmap(rises)
    }
}

#[cfg(test)]
mod tests {
    use std::env::temp_dir;

    use commons::noise::simplex_noise;

    use super::*;

    #[test]
    fn test() {
        // given
        let power = 8;
        let weights = (0..power + 1)
            .map(|i| 1.0f32 / 1.125f32.powf((power - i) as f32))
            .collect::<Vec<_>>();
        let rises = simplex_noise(power, 1987, &weights)
            .normalize()
            .map(|_, z| (0.5 - z).abs() / 0.5);

        let heightmap = rises_to_heightmap(rises);

        // when
        let with_valleys = heightmap.with_valleys(ValleyParameters {
            rain_threshold: 128,
            rise: 0.01,
        });

        // then
        let temp_path = temp_dir().join("test.png");
        let temp_path = temp_path.to_str().unwrap();
        with_valleys.to_image(temp_path).unwrap();

        let actual = image::open(temp_path).unwrap();
        let expected = image::open("test_resources/with_valleys/test.png").unwrap();
        assert_eq!(actual, expected);
    }
}
