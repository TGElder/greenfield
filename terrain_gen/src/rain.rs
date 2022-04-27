use commons::grid::Grid;
use commons::unsafe_float_ordering;

pub trait Rain {
    fn rain(&self) -> Grid<usize>;
}

impl Rain for Grid<Option<(i32, i32)>> {
    fn rain(&self) -> Grid<usize> {
        let mut xys = self.iter().collect::<Vec<_>>();
        xys.sort_by(|a, b| unsafe_float_ordering(&self[a], &self[b]));

        let mut out = Grid::default(self.width(), self.height());
        while let Some(xy) = xys.pop() {
            out[xy] += 1;
            if let Some(downhill) = self[xy] {
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
        #[rustfmt::skip]
        let downhills = Grid::from_vec(
            3,
            3,
            vec![
                None, Some((-1, 0)), Some((-1, 0)), // 
                Some((0, -1)), Some((0, 1)), Some((-1, 0)), // 
                Some((1, 0)), None, Some((-1, 0)), // 
            ]
        );

        assert_eq!(
            downhills.rain(),
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
