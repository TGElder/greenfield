use commons::geometry::{xy, XY};
use commons::grid::Grid;
use commons::unsafe_ordering::unsafe_ordering;

use crate::Heightmap;

pub type Downhills = Grid<Option<XY<i32>>>;

pub trait Downhill {
    fn downhills(&self) -> Downhills;
}

impl Downhill for Heightmap {
    fn downhills(&self) -> Downhills {
        self.map(|xy, _| lowest_neighbour_offset(self, &xy))
    }
}

fn lowest_neighbour(heightmap: &Heightmap, position: &XY<u32>) -> Option<XY<u32>> {
    heightmap
        .neighbours_4(position)
        .filter(|neighbour| heightmap[neighbour] < heightmap[position])
        .min_by(|a, b| unsafe_ordering(&heightmap[a], &heightmap[b]))
}

fn lowest_neighbour_offset(heightmap: &Heightmap, position: &XY<u32>) -> Option<XY<i32>> {
    let XY { x, y } = position;
    lowest_neighbour(heightmap, position).map(|n| {
        xy(
            (n.x as i64 - *x as i64) as i32,
            (n.y as i64 - *y as i64) as i32,
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let heightmap = Grid::from_vec(
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
            heightmap.downhills(), 
            Grid::from_vec(
                3,
                3,
                vec![
                    None, Some(xy(-1, 0)), Some(xy(-1, 0)), // 
                    Some(xy(0, -1)), Some(xy(0, 1)), Some(xy(-1, 0)), // 
                    Some(xy(1, 0)), None, Some(xy(-1, 0)), // 
                ]
            )
        );
    }
}
