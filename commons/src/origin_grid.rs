use std::borrow::Borrow;
use std::ops::{Index, IndexMut};

use serde::{Deserialize, Serialize};

use crate::geometry::{xy, Rectangle, XYRectangle, XY};
use crate::grid::Grid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OriginGrid<T> {
    origin: XY<u32>,
    grid: Grid<T>,
}

impl<T> OriginGrid<T> {
    pub fn origin(&self) -> &XY<u32> {
        &self.origin
    }

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

    pub fn rectangle(&self) -> Result<XYRectangle<u32>, String> {
        if self.width() == 0 || self.height() == 0 {
            return Err(format!(
                "XYRectangle cannot be computed for an OriginGrid with width ({}) == 0 or height ({}) == 0",
                self.width(),
                self.height()
            ));
        }
        Ok(XYRectangle {
            from: self.origin,
            to: self.origin + xy(self.width() - 1, self.height() - 1),
        })
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

    pub fn offset<B, C>(&self, position: B, offset: C) -> Option<XY<u32>>
    where
        B: Borrow<XY<u32>>,
        C: Borrow<XY<i32>>,
    {
        self.grid
            .offset(*position.borrow() - self.origin, offset)
            .map(|position| position + self.origin)
    }

    pub fn offsets<'a, B>(
        &'a self,
        position: B,
        offsets: &'a [XY<i32>],
    ) -> impl Iterator<Item = XY<u32>> + 'a
    where
        B: Borrow<XY<u32>> + Copy + 'a,
    {
        offsets.iter().flat_map(move |o| self.offset(position, o))
    }

    pub fn iter(&self) -> impl Iterator<Item = XY<u32>> {
        let origin = self.origin;
        self.grid.iter().map(move |xy| xy + origin)
    }

    pub fn map<F, U>(&self, mut function: F) -> OriginGrid<U>
    where
        F: FnMut(XY<u32>, &T) -> U,
    {
        OriginGrid {
            origin: self.origin,
            grid: self.grid.map(|xy, value| function(xy + self.origin, value)),
        }
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

        let Some((min, max)) = min_max else {
            return None;
        };

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

    use std::collections::HashSet;

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
    fn test_offset() {
        let grid = OriginGrid::new(xy(1, 2), Grid::<bool>::default(2, 3));

        assert_eq!(grid.offset(xy(2, 3), xy(-1, -1)), Some(xy(1, 2)));
    }

    #[test]
    fn test_offset_out_of_bounds_negative() {
        let grid = OriginGrid::new(xy(1, 2), Grid::<bool>::default(2, 3));

        assert_eq!(grid.offset(xy(1, 2), xy(-1, -1)), None);
    }

    #[test]
    fn test_offset_out_of_bounds_positive() {
        let grid = OriginGrid::new(xy(1, 2), Grid::<bool>::default(2, 3));

        assert_eq!(grid.offset(xy(3, 4), xy(1, 1)), None);
    }

    #[test]
    fn test_offsets() {
        let grid = OriginGrid::new(xy(1, 2), Grid::<bool>::default(2, 3));

        assert_eq!(
            grid.offsets(xy(3, 2), &[xy(-1, -1), xy(-1, 1), xy(1, 1)])
                .collect::<HashSet<_>>(),
            HashSet::from([xy(2, 3)])
        );
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
    fn test_map() {
        let grid = OriginGrid::new(xy(1, 2), Grid::from_fn(2, 3, |XY { x, y }| x + y));

        assert_eq!(
            grid.map(|XY { x, y }, value| x + y + value),
            OriginGrid::new(
                xy(1, 2),
                Grid::from_vec(
                    2,
                    3,
                    vec![
                        3, 5, //
                        5, 7, //
                        7, 9, //
                    ]
                )
            )
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

    #[test]
    fn test_rectangle() {
        // given
        let rectangle = XYRectangle {
            from: xy(1, 2),
            to: xy(3, 5),
        };

        let grid = OriginGrid::from_rectangle(rectangle, true);

        // when
        let result = grid.rectangle();

        // then
        assert_eq!(result, Ok(rectangle));
    }

    #[test]
    fn test_rectangle_zero_width_grid() {
        // given
        let grid = OriginGrid::new(xy(0, 0), Grid::from_element(0, 1, true));

        // when
        let result = grid.rectangle();

        // then
        assert_eq!(
            result,
            Err("XYRectangle cannot be computed for an OriginGrid with width (0) == 0 or height (1) == 0".to_string()),
        );
    }

    #[test]
    fn test_rectangle_zero_height_grid() {
        // given
        let grid = OriginGrid::new(xy(0, 0), Grid::from_element(1, 0, true));

        // when
        let result = grid.rectangle();

        // then
        assert_eq!(
            result,
            Err("XYRectangle cannot be computed for an OriginGrid with width (1) == 0 or height (0) == 0".to_string()),
        );
    }
}
