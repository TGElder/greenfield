use commons::grid::Grid;
use commons::unsafe_ordering;

pub trait Downhill {
    fn downhills(&self) -> Grid<Option<(i32, i32)>>;
}

impl Downhill for Grid<f32> {
    fn downhills(&self) -> Grid<Option<(i32, i32)>> {
        self.map(|xy, _| lowest_neighbour_offset(self, &xy))
    }
}

fn lowest_neighbour(grid: &Grid<f32>, xy: &(u32, u32)) -> Option<(u32, u32)> {
    grid.neighbours_4(xy)
        .filter(|neighbour| grid[neighbour] < grid[xy])
        .min_by(|a, b| unsafe_ordering(&grid[a], &grid[b]))
}

fn lowest_neighbour_offset(grid: &Grid<f32>, xy: &(u32, u32)) -> Option<(i32, i32)> {
    let (x, y) = xy;
    lowest_neighbour(grid, xy).map(|(nx, ny)| {
        (
            (nx as i64 - *x as i64) as i32,
            (ny as i64 - *y as i64) as i32,
        )
    })
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

        #[rustfmt::skip]
        assert_eq!(
            grid.downhills(), 
            Grid::from_vec(
                3,
                3,
                vec![
                    None, Some((-1, 0)), Some((-1, 0)), // 
                    Some((0, -1)), Some((0, 1)), Some((-1, 0)), // 
                    Some((1, 0)), None, Some((-1, 0)), // 
                ]
            )
        );
    }
}
