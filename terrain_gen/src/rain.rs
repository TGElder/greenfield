use commons::grid::Grid;
use commons::unsafe_float_ordering;

use crate::Downhill;

pub trait Rain {
    fn rain(&self) -> Grid<usize>;
}

impl Rain for Grid<f32> {
    fn rain(&self) -> Grid<usize> {
        let downhills = self.downhills();

        let mut xys = self.iter().collect::<Vec<_>>();
        xys.sort_by(|a, b| unsafe_float_ordering(&self[a], &self[b]));

        let mut out = Grid::default(self.width(), self.height());
        while let Some(xy) = xys.pop() {
            out[xy] += 1;
            if let Some(downhill) = downhills[xy] {
                let downhill_xy = self.offset(xy, downhill).unwrap();
                out[downhill_xy] += out[xy];
            }
        }

        out
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        let grid = Grid::from_vec(
            3,
            3,
            vec![
                0.0, 0.1, 4.0, //
                3.0, 0.2, 3.0, //
                0.1, 0.0, 4.0, //
            ],
        );

        assert_eq!(
            grid.rain(),
            Grid::from_vec(
                3,
                3,
                vec![
                    4, 2, 1, //
                    1, 2, 1, //
                    1, 5, 1, //
                ],
            )
        );
    }
}
