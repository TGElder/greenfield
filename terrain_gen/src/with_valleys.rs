use std::borrow::Borrow;

use crate::{Heightmap, Rain, ToHeightmap, ToRises};

pub struct ValleyParameters {
    rain_threshold: usize,
    rise: f32,
}

pub trait WithValleys {
    fn with_valleys<B>(&self, parameters: B) -> Heightmap
    where
        B: Borrow<ValleyParameters>;
}

impl WithValleys for Heightmap {
    fn with_valleys<B>(&self, parameters: B) -> Heightmap
    where
        B: Borrow<ValleyParameters>,
    {
        let parameters = parameters.borrow();

        let rain = self.rain();

        let rises = self
            .map(|xy, z| {
                if rain[xy] > parameters.rain_threshold {
                    parameters.rise
                } else {
                    *z
                }
            })
            .to_rises();

        rises.to_heightmap()
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
            .map(|_, z| (0.5 - z).abs() / 0.5)
            .to_rises();

        let heightmap = rises.to_heightmap();

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