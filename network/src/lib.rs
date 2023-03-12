pub mod algorithms;
pub mod model;

#[cfg(test)]
mod tests {
    use commons::geometry::{xy, XY};
    use commons::grid::Grid;

    use crate::model::{Edge, Network};

    use super::*;

    #[test]
    fn test_name() {
        let grid: Grid<()> = Grid::default(8, 8);

        struct GridRef<'a, T> {
            grid: &'a Grid<T>,
            index: usize,
        }

        impl Network for Grid<u32> {
            fn edges<'a>(&'a self, index: &'a usize) -> Box<dyn Iterator<Item = Edge> + 'a> {
                let XY { x, y } = self.xy(index);
                Box::new(
                    self.neighbours_4(xy(x as u32, y as u32))
                        .map(|n| self.index(n))
                        .map(|to| Edge {
                            from: *index,
                            to,
                            cost: 1,
                        }),
                )
            }
        }
    }
}
