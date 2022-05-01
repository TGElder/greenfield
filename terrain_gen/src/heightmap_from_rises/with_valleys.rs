pub mod with_valleys;

use std::borrow::Borrow;

use commons::scale::Scale;

use crate::{heightmap_from_rises, Heightmap, Rain, Rises};

pub struct ValleyParameters<F>
where
    F: Fn((u32, u32)) -> bool,
{
    pub height_threshold: f32,
    pub rain_threshold: usize,
    pub rise: f32,
    pub origin_fn: F,
}

pub fn heightmap_from_rises_with_valleys<B, F>(rises: &Rises, parameters: B) -> Heightmap
where
    B: Borrow<ValleyParameters<F>>,
    F: Fn((u32, u32)) -> bool,
{
    let parameters = parameters.borrow();

    let heightmap = heightmap_from_rises(rises, &parameters.origin_fn);
    let rain = heightmap.rain();

    let valley_rises = rises.map(|xy, z| {
        if rain[xy] > parameters.rain_threshold {
            parameters.rise
        } else {
            *z
        }
    });
    let valley_heightmap = heightmap_from_rises(&valley_rises, |xy| {
        heightmap[xy] <= parameters.height_threshold
    });

    let valley_scale = Scale::new((0.0, 1.0), (parameters.height_threshold, 1.0));
    heightmap.map(|xy, z| {
        if *z > parameters.height_threshold {
            valley_scale.scale(valley_heightmap[xy])
        } else {
            heightmap[xy]
        }
    })
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

        // when
        let heightmap = heightmap_from_rises_with_valleys(
            &rises,
            ValleyParameters {
                height_threshold: 0.25,
                rain_threshold: 128,
                rise: 0.01,
                origin_fn: |xy| rises.is_border(xy),
            },
        );

        // then
        let temp_path = temp_dir().join("test.png");
        let temp_path = temp_path.to_str().unwrap();
        heightmap.to_image(temp_path).unwrap();

        let actual = image::open(temp_path).unwrap();
        let expected =
            image::open("test_resources/heightmap_from_rises/with_valleys/test.png").unwrap();
        assert_eq!(actual, expected);
    }
}
