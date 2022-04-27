use commons::grid::Grid;
use commons::unsafe_float_ordering;

use crate::Heightmap;

pub type Downhills = Grid<Option<(i32, i32)>>;

pub trait Downhill {
    fn downhills(&self) -> Downhills;
}

impl Downhill for Heightmap {
    fn downhills(&self) -> Downhills {
        self.map(|xy, _| lowest_neighbour_offset(self, &xy))
    }
}

fn lowest_neighbour(heightmap: &Heightmap, xy: &(u32, u32)) -> Option<(u32, u32)> {
    heightmap
        .neighbours_4(xy)
        .filter(|neighbour| heightmap[neighbour] < heightmap[xy])
        .min_by(|a, b| unsafe_float_ordering(&heightmap[a], &heightmap[b]))
}

fn lowest_neighbour_offset(heightmap: &Heightmap, xy: &(u32, u32)) -> Option<(i32, i32)> {
    let (x, y) = xy;
    lowest_neighbour(heightmap, xy).map(|(nx, ny)| {
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
                    None, Some((-1, 0)), Some((-1, 0)), // 
                    Some((0, -1)), Some((0, 1)), Some((-1, 0)), // 
                    Some((1, 0)), None, Some((-1, 0)), // 
                ]
            )
        );
    }
}
