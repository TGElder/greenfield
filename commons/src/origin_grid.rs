use std::borrow::Borrow;
use std::ops::{Index, IndexMut};

use crate::geometry::{xy, XYRectangle, XY};
use crate::grid::Grid;

#[derive(Debug)]
pub struct OriginGrid<T> {
    origin: XY<u32>,
    grid: Grid<T>,
}

impl<T> OriginGrid<T> {
    pub fn width(&self) -> u32 {
        self.grid.width()
    }

    pub fn height(&self) -> u32 {
        self.grid.height()
    }

    pub fn new(origin: XY<u32>, grid: Grid<T>) -> OriginGrid<T> {
        OriginGrid { origin, grid }
    }

    pub fn from_rectangle<B>(rectangle: B, element: T) -> OriginGrid<T>
    where
        B: Borrow<XYRectangle<u32>>,
        T: Clone,
    {
        let rectangle = rectangle.borrow();
        OriginGrid {
            origin: rectangle.from,
            grid: Grid::from_element(rectangle.width(), rectangle.height(), element),
        }
    }

    pub fn index<B>(&self, position: B) -> usize
    where
        B: Borrow<XY<u32>>,
    {
        self.grid.index(*position.borrow() - self.origin)
    }

    pub fn xy<B>(&self, index: B) -> XY<u32>
    where
        B: Borrow<usize>,
    {
        self.grid.xy(index.borrow()) + xy(self.origin.x, self.origin.y)
    }

    pub fn in_bounds<B>(&self, position: B) -> bool
    where
        B: Borrow<XY<u32>>,
    {
        let position = position.borrow();
        if self.origin.x > position.x || self.origin.y > position.y {
            return false;
        }
        self.grid.in_bounds(*position - self.origin)
    }

    pub fn iter(&self) -> impl Iterator<Item = XY<u32>> + '_ {
        self.grid.iter().map(|xy| xy + self.origin)
    }
}

impl<T> PartialEq for OriginGrid<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.grid == other.grid
    }
}

impl<B, T> Index<B> for OriginGrid<T>
where
    B: Borrow<XY<u32>>,
{
    type Output = T;

    fn index(&self, index: B) -> &Self::Output {
        &self.grid[*index.borrow() - self.origin]
    }
}

impl<B, T> IndexMut<B> for OriginGrid<T>
where
    B: Borrow<XY<u32>>,
{
    fn index_mut(&mut self, index: B) -> &mut Self::Output {
        &mut self.grid[*index.borrow() - self.origin]
    }
}

impl<T> OriginGrid<T>
where
    T: Copy + Default + Ord,
{
    pub fn paste(&self, other: &OriginGrid<T>) -> OriginGrid<T> {
        let min = xy(
            self.origin.x.min(other.origin.x),
            self.origin.y.min(other.origin.y),
        );
        let max_plus_one = xy(
            (self.origin.x + self.width()).max(other.origin.x + other.width()),
            (self.origin.y + self.height()).max(other.origin.y + other.height()),
        );

        let mut out = OriginGrid {
            origin: min,
            grid: Grid::default(max_plus_one.x - min.x, max_plus_one.y - min.y),
        };
        self.iter().for_each(|xy| out[xy] = self[xy]);
        other.iter().for_each(|xy| out[xy] = other[xy]);

        out
    }
}

impl<T> OriginGrid<T>
where
    T: Copy + Default + PartialEq,
{
    pub fn crop(&self) -> Option<OriginGrid<T>> {
        let mut min_max: Option<(XY<u32>, XY<u32>)> = None;

        self.iter().filter(|xy| self[xy] != T::default()).for_each(
            |XY { x, y }| match &mut min_max {
                Some((min, max)) => {
                    min.x = min.x.min(x);
                    max.x = max.x.max(x);
                    min.y = min.y.min(y);
                    max.y = max.y.max(y);
                }
                None => min_max = Some((xy(x, y), xy(x, y))),
            },
        );

        let Some((min, max)) = min_max else {return None};

        let mut out = OriginGrid {
            origin: min,
            grid: Grid::default((max.x - min.x) + 1, (max.y - min.y) + 1),
        };

        for x in min.x..=max.x {
            for y in min.y..=max.y {
                let xy = xy(x, y);
                out[xy] = self[xy];
            }
        }

        Some(out)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_width() {
        let grid = OriginGrid::new(xy(1, 2), Grid::<bool>::default(4, 5));

        assert_eq!(grid.width(), 4);
    }

    #[test]
    fn test_height() {
        let grid = OriginGrid::new(xy(1, 2), Grid::<bool>::default(4, 5));

        assert_eq!(grid.height(), 5);
    }

    #[test]
    fn test_index() {
        let grid = OriginGrid::new(xy(1, 2), Grid::from_element(2, 3, false));

        assert_eq!(grid.index(xy(1, 2)), 0);
        assert_eq!(grid.index(xy(2, 2)), 1);
        assert_eq!(grid.index(xy(1, 3)), 2);
        assert_eq!(grid.index(xy(2, 3)), 3);
        assert_eq!(grid.index(xy(1, 4)), 4);
        assert_eq!(grid.index(xy(2, 4)), 5);
    }

    #[test]
    fn test_index_traits() {
        let mut grid = OriginGrid::new(xy(1, 2), Grid::from_element(2, 3, false));

        grid[xy(2, 4)] = true;

        assert!(grid[xy(2, 4)]);
    }

    #[test]
    fn test_xy() {
        let grid = OriginGrid::new(xy(1, 2), Grid::from_element(2, 3, false));

        assert_eq!(grid.xy(0), xy(1, 2));
        assert_eq!(grid.xy(1), xy(2, 2));
        assert_eq!(grid.xy(2), xy(1, 3));
        assert_eq!(grid.xy(3), xy(2, 3));
        assert_eq!(grid.xy(4), xy(1, 4));
        assert_eq!(grid.xy(5), xy(2, 4));
    }

    #[test]
    fn test_in_bounds() {
        let grid = OriginGrid::new(xy(1, 2), Grid::from_element(2, 3, false));

        assert!(!grid.in_bounds(xy(0, 1)));
        assert!(!grid.in_bounds(xy(1, 1)));
        assert!(!grid.in_bounds(xy(2, 1)));
        assert!(!grid.in_bounds(xy(3, 1)));
        assert!(!grid.in_bounds(xy(0, 2)));
        assert!(grid.in_bounds(xy(1, 2)));
        assert!(grid.in_bounds(xy(2, 2)));
        assert!(!grid.in_bounds(xy(3, 2)));
        assert!(!grid.in_bounds(xy(0, 3)));
        assert!(grid.in_bounds(xy(1, 3)));
        assert!(grid.in_bounds(xy(2, 3)));
        assert!(!grid.in_bounds(xy(3, 3)));
        assert!(!grid.in_bounds(xy(0, 4)));
        assert!(grid.in_bounds(xy(1, 4)));
        assert!(grid.in_bounds(xy(2, 4)));
        assert!(!grid.in_bounds(xy(3, 4)));
        assert!(!grid.in_bounds(xy(0, 5)));
        assert!(!grid.in_bounds(xy(1, 5)));
        assert!(!grid.in_bounds(xy(2, 5)));
        assert!(!grid.in_bounds(xy(3, 5)));
    }

    #[test]
    fn test_iter_2x3() {
        let grid = OriginGrid::new(xy(1, 2), Grid::<bool>::default(2, 3));

        assert_eq!(
            grid.iter().collect::<Vec<_>>(),
            vec![xy(1, 2), xy(2, 2), xy(1, 3), xy(2, 3), xy(1, 4), xy(2, 4),]
        );
    }

    #[test]
    fn test_paste() {
        let grid_1 = OriginGrid::new(xy(2, 4), Grid::from_element(3, 2, 1));
        let grid_2 = OriginGrid::new(xy(1, 2), Grid::from_element(2, 3, 2));

        assert_eq!(
            grid_1.paste(&grid_2),
            OriginGrid::new(
                xy(1, 2),
                Grid::from_vec(
                    4,
                    4,
                    vec![
                        2, 2, 0, 0, //
                        2, 2, 0, 0, //
                        2, 2, 1, 1, //
                        0, 1, 1, 1, //
                    ]
                )
            )
        );
    }

    #[test]
    fn test_crop() {
        let grid = OriginGrid::new(
            xy(1, 2),
            Grid::from_vec(
                4,
                5,
                vec![
                    0, 0, 0, 0, //
                    0, 1, 0, 0, //
                    0, 0, 1, 0, //
                    0, 1, 0, 0, //
                    0, 0, 0, 0, //
                ],
            ),
        );

        assert_eq!(
            grid.crop(),
            Some(OriginGrid::new(
                xy(2, 3),
                Grid::from_vec(
                    2,
                    3,
                    vec![
                        1, 0, //
                        0, 1, //
                        1, 0, //
                    ]
                )
            ))
        );
    }

    #[test]
    fn test_crop_empty() {
        let grid = OriginGrid::new(
            xy(1, 2),
            Grid::from_vec(
                2,
                2,
                vec![
                    0, 0, //
                    0, 0, //
                ],
            ),
        );

        assert_eq!(grid.crop(), None);
    }

    #[test]
    fn test_from_rectangle() {
        // given
        let rectangle = XYRectangle {
            from: xy(1, 2),
            to: xy(3, 5),
        };

        // when
        let result = OriginGrid::from_rectangle(rectangle, true);

        // then
        assert_eq!(
            result,
            OriginGrid::new(xy(1, 2), Grid::from_element(3, 4, true))
        );
    }
}
