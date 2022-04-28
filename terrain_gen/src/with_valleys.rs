use std::borrow::Borrow;

use commons::scale::Scale;

use crate::{AsHeightmap, AsRises, Heightmap, Rain};

pub struct ValleyParameters {
    height_threshold: f32,
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

        let noise = self.map(|xy, z| {
            if rain[xy] > parameters.rain_threshold {
                parameters.rise
            } else {
                *z
            }
        });

        let valleys = noise
            .as_rises()
            .as_heightmap(|xy| self[xy] <= parameters.height_threshold);

        let valleys_scale = Scale::new((0.0, 1.0), (parameters.height_threshold, 1.0));
        self.map(|xy, z| {
            if *z > parameters.height_threshold {
                valleys_scale.scale(valleys[xy])
            } else {
                self[xy]
            }
        })
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
        let noise = simplex_noise(power, 1987, &weights)
            .normalize()
            .map(|_, z| (0.5 - z).abs() / 0.5);

        let heightmap = noise.as_rises().as_heightmap(|xy| noise.is_border(xy));

        // when
        let with_valleys = heightmap.with_valleys(ValleyParameters {
            height_threshold: 0.25,
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
