use image::{ImageBuffer, Luma};
use num::Float;

use crate::grid::Grid;
use crate::scale::Scale;
use crate::unsafe_ordering;

impl<T> Grid<T>
where
    T: Float,
{
    pub fn min(&self) -> T {
        *self
            .iter()
            .map(|xy| &self[xy])
            .min_by(unsafe_ordering)
            .unwrap()
    }

    pub fn max(&self) -> T {
        *self
            .iter()
            .map(|xy| &self[xy])
            .max_by(unsafe_ordering)
            .unwrap()
    }

    pub fn normalize(&self) -> Grid<T> {
        let scale = Scale::new((self.min(), self.max()), (T::zero(), T::one()));
        self.map(|_, value| scale.scale(value))
    }

    pub fn to_image(&self, file_name: &str) {
        let min = self.min();
        let mut max = self.max();
        if min == max {
            max = T::one(); // avoids divide by zero, results in black output
        }
        let scale = Scale::new((min, max), (T::zero(), T::one()));

        let image = ImageBuffer::from_fn(self.width(), self.height(), |x, y| {
            let scaled = scale.scale(self[(x, y)]);

            let luma = scaled * T::from(255u8).unwrap();
            let luma = luma.round();
            let luma = luma.to_u8().unwrap();
            Luma([luma])
        });

        image.save(file_name).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use std::env::temp_dir;

    use super::*;

    #[test]
    fn test_min() {
        let grid = Grid::from_fn(3, 3, |(x, y)| (x + y) as f32 + 1.0);

        assert_eq!(grid.min(), 1.0);
    }

    #[test]
    fn test_max() {
        let grid = Grid::from_fn(3, 3, |(x, y)| (x + y) as f32 + 1.0);

        assert_eq!(grid.max(), 5.0);
    }

    #[test]
    fn test_normalize() {
        let grid = Grid::from_vec(
            3,
            3,
            vec![
                1.0, 3.0, 5.0, //
                3.0, 5.0, 9.0, //
                5.0, 9.0, 17.0, //
            ],
        );

        assert_eq!(
            grid.normalize(),
            Grid::from_vec(
                3,
                3,
                vec![
                    0.0, 0.125, 0.25, //
                    0.125, 0.25, 0.5, //
                    0.25, 0.5, 1.0 //
                ]
            )
        );
    }

    #[test]
    fn test_to_image() {
        // given
        let grid = Grid::from_fn(128, 128, |(x, y)| (x + y) as f32 + 1.0);

        // when
        let temp_path = temp_dir().join("test_to_image.png");
        let temp_path = temp_path.to_str().unwrap();
        grid.to_image(temp_path);

        // then
        let actual = image::open(temp_path).unwrap();
        let expected = image::open("test_resources/float_grid/test_to_image.png").unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_all_zeros_to_image() {
        let grid = Grid::<f32>::default(128, 128);

        // when
        let temp_path = temp_dir().join("test_all_zeros_to_image.png");
        let temp_path = temp_path.to_str().unwrap();
        grid.to_image(temp_path);

        // then
        let actual = image::open(temp_path).unwrap();
        let expected =
            image::open("test_resources/float_grid/test_all_zeros_to_image.png").unwrap();
        assert_eq!(actual, expected);
    }
}
