use std::borrow::Borrow;
use std::ops::{Add, Index, IndexMut, Sub};

use crate::geometry::{xy, XY};

#[derive(Debug)]
pub struct Grid<T> {
    width: u32,
    height: u32,
    elements: Vec<T>,
}

impl<T> Grid<T> {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn from_vec(width: u32, height: u32, elements: Vec<T>) -> Grid<T> {
        let len = width as usize * height as usize;
        if elements.len() != len {
            panic!(
                "Number of elements {} does not equal (width={} * height={}) = {}",
                elements.len(),
                width,
                height,
                len
            );
        }
        Grid {
            width,
            height,
            elements,
        }
    }

    pub fn from_fn<F>(width: u32, height: u32, mut function: F) -> Grid<T>
    where
        F: FnMut(XY<u32>) -> T,
    {
        let mut elements = Vec::with_capacity(width as usize * height as usize);
        for y in 0..height {
            for x in 0..width {
                elements.push(function(xy(x, y)));
            }
        }
        Grid {
            width,
            height,
            elements,
        }
    }

    pub fn index<B>(&self, position: B) -> usize
    where
        B: Borrow<XY<u32>>,
    {
        let XY { x, y } = position.borrow();
        (*x as usize) + (*y as usize) * self.width as usize
    }

    pub fn xy<B>(&self, index: B) -> XY<u32>
    where
        B: Borrow<usize>,
    {
        let index = index.borrow();
        xy(
            (index % self.width as usize) as u32, // result must be smaller than self.width and so smaller than u32::MAX
            (index / self.width as usize)
                .try_into()
                .unwrap_or_else(|_| {
                    panic!(
                        "y must be <= {} but would be {} for index {} in grid with width {}",
                        u32::MAX,
                        index / self.height as usize,
                        index,
                        self.height
                    )
                }),
        )
    }

    pub fn in_bounds<B>(&self, position: B) -> bool
    where
        B: Borrow<XY<u32>>,
    {
        let XY { x, y } = position.borrow();
        *x < self.width && *y < self.height
    }

    pub fn is_border<B>(&self, position: B) -> bool
    where
        B: Borrow<XY<u32>>,
    {
        let XY { x, y } = position.borrow();
        *x == 0 || *y == 0 || *x == self.width - 1 || *y == self.height - 1
    }

    pub fn map<F, U>(&self, mut function: F) -> Grid<U>
    where
        F: FnMut(XY<u32>, &T) -> U,
    {
        let mut elements = Vec::with_capacity(self.elements.len());
        let mut index = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                elements.push(function(XY { x, y }, &self.elements[index]));
                index += 1;
            }
        }
        Grid {
            width: self.width,
            height: self.height,
            elements,
        }
    }

    pub fn offset<B, C>(&self, position: B, offset: C) -> Option<XY<u32>>
    where
        B: Borrow<XY<u32>>,
        C: Borrow<XY<i32>>,
    {
        let XY { x, y } = position.borrow();
        let d = offset.borrow();

        let nx = (*x) as i64 + d.x as i64;
        let ny = (*y) as i64 + d.y as i64;

        let nx = nx.try_into().ok()?;
        let ny = ny.try_into().ok()?;

        if !self.in_bounds(xy(nx, ny)) {
            return None;
        }

        Some(xy(nx, ny))
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

    pub fn neighbours_4<'a, B>(&'a self, position: B) -> impl Iterator<Item = XY<u32>> + 'a
    where
        B: Borrow<XY<u32>> + Copy + 'a,
    {
        const OFFSETS_4: [XY<i32>; 4] = [xy(1, 0), xy(0, 1), xy(-1, 0), xy(0, -1)];
        self.offsets(position, &OFFSETS_4)
    }

    pub fn neighbours_8<'a, B>(&'a self, position: B) -> impl Iterator<Item = XY<u32>> + 'a
    where
        B: Borrow<XY<u32>> + Copy + 'a,
    {
        const OFFSETS_8: [XY<i32>; 8] = [
            xy(1, 0),
            xy(1, 1),
            xy(0, 1),
            xy(-1, 1),
            xy(-1, 0),
            xy(-1, -1),
            xy(0, -1),
            xy(1, -1),
        ];
        self.offsets(position, &OFFSETS_8)
    }

    pub fn iter(&self) -> impl Iterator<Item = XY<u32>> + '_ {
        (0..self.height).flat_map(|y| (0..self.width).map(move |x| XY { x, y }))
    }
}

impl<T> Grid<T>
where
    T: Default,
{
    pub fn default(width: u32, height: u32) -> Grid<T> {
        Grid {
            elements: (0..(width as usize * height as usize))
                .map(|_| T::default())
                .collect(),
            width,
            height,
        }
    }
}

impl<T> Grid<T>
where
    T: Clone,
{
    pub fn from_element(width: u32, height: u32, element: T) -> Grid<T> {
        Grid {
            elements: vec![element; width as usize * height as usize],
            width,
            height,
        }
    }
}

impl<T> PartialEq for Grid<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height && self.elements == other.elements
    }
}

impl<B, T> Index<B> for Grid<T>
where
    B: Borrow<XY<u32>>,
{
    type Output = T;

    fn index(&self, index: B) -> &Self::Output {
        &self.elements[self.index(index)]
    }
}

impl<B, T> IndexMut<B> for Grid<T>
where
    B: Borrow<XY<u32>>,
{
    fn index_mut(&mut self, index: B) -> &mut Self::Output {
        let index = self.index(index);
        &mut self.elements[index]
    }
}

impl<T> Add for Grid<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Grid<T>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.width != rhs.width || self.height != rhs.height {
            panic!(
                "Trying to add grid with dimensions {}x{} to grid with dimensions {}x{}",
                self.width, self.height, rhs.width, rhs.height
            );
        }
        self.map(|xy, value| *value + rhs[xy])
    }
}

impl<T> Sub for Grid<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Grid<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.width != rhs.width || self.height != rhs.height {
            panic!(
                "Trying to subtract grid with dimensions {}x{} from grid with dimensions {}x{}",
                rhs.width, rhs.height, self.width, self.height
            );
        }

        self.map(|xy, value| *value - rhs[xy])
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use maplit::hashset;

    use super::*;

    #[test]
    fn test_width() {
        let grid = Grid::<bool>::default(4, 5);

        assert_eq!(grid.width(), 4);
    }

    #[test]
    fn test_height() {
        let grid = Grid::<bool>::default(4, 5);

        assert_eq!(grid.height(), 5);
    }

    #[test]
    fn test_default() {
        let grid = Grid::<bool>::default(4, 5);

        assert_eq!(
            grid,
            Grid {
                width: 4,
                height: 5,
                elements: vec![false; 20],
            }
        );
    }

    // Too slow
    // #[test]
    // fn test_default_max_u32() {
    //     Grid::<bool>::default(u32::MAX, 2);
    // }

    #[test]
    fn test_from_element() {
        let grid = Grid::from_element(4, 5, Some(3));

        assert_eq!(
            grid,
            Grid {
                width: 4,
                height: 5,
                elements: vec![Some(3); 20],
            }
        );
    }

    #[test]
    fn test_from_element_max_u32() {
        Grid::from_element(u32::MAX, 2, false);
    }

    #[test]
    fn test_from_vec() {
        let grid = Grid::from_vec(
            2,
            3,
            vec![
                (0, 0),
                (1, 0), //
                (0, 1),
                (1, 1), //
                (0, 2),
                (1, 2), //
            ],
        );

        assert_eq!(
            grid,
            Grid {
                width: 2,
                height: 3,
                elements: vec![
                    (0, 0),
                    (1, 0), //
                    (0, 1),
                    (1, 1), //
                    (0, 2),
                    (1, 2), //
                ]
            }
        );
    }

    #[test]
    fn test_from_vec_max_u32() {
        Grid::from_vec(u32::MAX, 2, vec![false; u32::MAX as usize * 2]);
    }

    #[test]
    #[should_panic(expected = "Number of elements 1 does not equal (width=2 * height=3) = 6")]
    fn test_from_vec_of_wrong_length() {
        Grid::from_vec(2, 3, vec![false]);
    }

    #[test]
    fn test_from_fn() {
        let grid = Grid::from_fn(2, 3, |t| t);

        #[rustfmt::skip]
        assert_eq!(
            grid,
            Grid {
                width: 2,
                height: 3,
                elements: vec![
                    xy(0, 0), xy(1, 0),
                    xy(0, 1), xy(1, 1),
                    xy(0, 2), xy(1, 2),
                ]
            }
        );
    }

    // Too slow
    // #[test]
    // fn test_from_fn_max_u32() {
    //     Grid::from_fn(u32::MAX, 2, |_| false);
    // }

    #[test]
    fn test_index() {
        let grid = Grid::from_element(2, 3, false);

        assert_eq!(grid.index(xy(0, 0)), 0);
        assert_eq!(grid.index(xy(1, 0)), 1);
        assert_eq!(grid.index(xy(0, 1)), 2);
        assert_eq!(grid.index(xy(1, 1)), 3);
        assert_eq!(grid.index(xy(0, 2)), 4);
        assert_eq!(grid.index(xy(1, 2)), 5);
    }

    #[test]
    fn test_index_traits() {
        let mut grid = Grid::from_element(2, 3, false);

        grid[xy(1, 2)] = true;

        assert!(grid[xy(1, 2)]);
    }

    #[test]
    fn test_xy() {
        let grid = Grid::from_element(2, 3, false);

        assert_eq!(grid.xy(0), xy(0, 0));
        assert_eq!(grid.xy(1), xy(1, 0));
        assert_eq!(grid.xy(2), xy(0, 1));
        assert_eq!(grid.xy(3), xy(1, 1));
        assert_eq!(grid.xy(4), xy(0, 2));
        assert_eq!(grid.xy(5), xy(1, 2));
    }

    #[test]
    #[should_panic(
        expected = "y must be <= 4294967295 but would be 4294967296 for index 4294967296 in grid with width 1"
    )]
    fn test_xy_y_too_large() {
        let grid = Grid::<bool>::default(1, 1);

        grid.xy(4294967296);
    }

    #[test]
    fn test_in_bounds() {
        let grid = Grid::from_element(2, 3, false);

        assert!(grid.in_bounds(xy(0, 0)));
        assert!(grid.in_bounds(xy(1, 0)));
        assert!(!grid.in_bounds(xy(2, 0)));
        assert!(grid.in_bounds(xy(0, 1)));
        assert!(grid.in_bounds(xy(1, 1)));
        assert!(!grid.in_bounds(xy(2, 1)));
        assert!(grid.in_bounds(xy(0, 2)));
        assert!(grid.in_bounds(xy(1, 2)));
        assert!(!grid.in_bounds(xy(2, 2)));
        assert!(!grid.in_bounds(xy(0, 3)));
        assert!(!grid.in_bounds(xy(1, 3)));
        assert!(!grid.in_bounds(xy(2, 3)));
    }

    #[test]
    fn test_is_border() {
        let grid = Grid::from_element(3, 3, false);

        assert!(grid.is_border(xy(0, 0)));
        assert!(grid.is_border(xy(1, 0)));
        assert!(grid.is_border(xy(2, 0)));
        assert!(grid.is_border(xy(0, 1)));
        assert!(!grid.is_border(xy(1, 1)));
        assert!(grid.is_border(xy(2, 1)));
        assert!(grid.is_border(xy(0, 2)));
        assert!(grid.is_border(xy(1, 2)));
        assert!(grid.is_border(xy(2, 2)));
    }

    #[test]
    fn test_map() {
        let grid = Grid::from_element(2, 3, 1);

        assert_eq!(
            grid.map(|XY { x, y }, z| x + y + z),
            Grid {
                width: 2,
                height: 3,
                elements: vec![
                    1, 2, //
                    2, 3, //
                    3, 4, //
                ]
            }
        );
    }

    #[test]
    fn test_offset() {
        let grid = Grid::from_element(2, 3, false);

        assert_eq!(grid.offset(xy(1, 1), xy(-1, 1)), Some(xy(0, 2)));
    }

    #[test]
    fn test_offset_out_of_bounds_negative() {
        let grid = Grid::from_element(2, 3, false);

        assert_eq!(grid.offset(xy(0, 1), xy(-1, 1)), None);
    }

    #[test]
    fn test_offset_out_of_bounds_positive() {
        let grid = Grid::from_element(2, 3, false);

        assert_eq!(grid.offset(xy(1, 1), xy(2, 1)), None);
    }

    #[test]
    fn test_offset_with_max_u32() {
        let grid = Grid::from_element(u32::MAX, 2, false);

        assert_eq!(
            grid.offset(xy(u32::MAX - 2, 0), xy(1, 1)),
            Some(xy(u32::MAX - 1, 1))
        );
    }

    #[test]
    fn test_offset_with_max_i32() {
        let grid = Grid::from_element(u32::MAX, 1, false);

        assert_eq!(
            grid.offset(xy(u32::MAX - i32::MAX as u32 - 1, 0), xy(i32::MAX, 0)),
            Some(xy(u32::MAX - 1, 0))
        );
    }

    #[test]
    fn test_offset_with_min_i32() {
        let grid = Grid::from_element(u32::MAX, 1, false);

        assert_eq!(
            grid.offset(xy(i32::MAX as u32 + 1, 0), xy(i32::MIN, 0)),
            Some(xy(0, 0))
        );
    }

    #[test]
    fn test_offset_overflow() {
        let grid = Grid::from_element(u32::MAX, 2, false);

        assert_eq!(grid.offset(xy(u32::MAX, 0), xy(i32::MAX, 1)), None);
    }

    #[test]
    fn test_offsets() {
        let grid = Grid::from_element(2, 3, false);

        assert_eq!(
            grid.offsets(xy(0, 1), &[xy(1, 0), xy(0, 1), xy(-1, -1)])
                .collect::<HashSet<_>>(),
            hashset! {xy(1, 1), xy(0, 2)}
        );
    }

    #[test]
    fn test_neighbours_4_in_middle() {
        let grid = Grid::from_element(3, 3, false);

        assert_eq!(
            grid.neighbours_4(xy(1, 1)).collect::<HashSet<_>>(),
            hashset! {xy(2, 1), xy(1, 2), xy(0, 1), xy(1, 0)}
        );
    }

    #[test]
    fn test_neighbours_4_in_corner() {
        let grid = Grid::from_element(3, 3, false);

        assert_eq!(
            grid.neighbours_4(xy(0, 0)).collect::<HashSet<_>>(),
            hashset! {xy(1, 0), xy(0, 1)}
        );
    }

    #[test]
    fn test_neighbours_8_in_middle() {
        let grid = Grid::from_element(3, 3, false);

        assert_eq!(
            grid.neighbours_8(xy(1, 1)).collect::<HashSet<_>>(),
            hashset! {xy(2, 1), xy(2, 2), xy(1, 2), xy(0, 2), xy(0, 1), xy(0, 0), xy(1, 0), xy(2, 0)}
        );
    }

    #[test]
    fn test_neighbours_8_in_corner() {
        let grid = Grid::from_element(3, 3, false);

        assert_eq!(
            grid.neighbours_4(xy(0, 0)).collect::<HashSet<_>>(),
            hashset! {xy(1, 0), xy(0, 1)}
        );
    }

    #[test]
    fn test_iter_0x0() {
        let grid = Grid::<bool>::default(0, 0);

        assert_eq!(grid.iter().collect::<Vec<_>>(), vec![]);
    }

    #[test]
    fn test_iter_0x1() {
        let grid = Grid::<bool>::default(0, 1);

        assert_eq!(grid.iter().collect::<Vec<_>>(), vec![]);
    }

    #[test]
    fn test_iter_0x2() {
        let grid = Grid::<bool>::default(0, 2);

        assert_eq!(grid.iter().collect::<Vec<_>>(), vec![]);
    }

    #[test]
    fn test_iter_0x3() {
        let grid = Grid::<bool>::default(0, 3);

        assert_eq!(grid.iter().collect::<Vec<_>>(), vec![]);
    }

    #[test]
    fn test_iter_1x0() {
        let grid = Grid::<bool>::default(1, 0);

        assert_eq!(grid.iter().collect::<Vec<_>>(), vec![]);
    }

    #[test]
    fn test_iter_1x1() {
        let grid = Grid::<bool>::default(1, 1);

        assert_eq!(grid.iter().collect::<Vec<_>>(), vec![xy(0, 0),]);
    }

    #[test]
    fn test_iter_1x2() {
        let grid = Grid::<bool>::default(1, 2);

        assert_eq!(grid.iter().collect::<Vec<_>>(), vec![xy(0, 0), xy(0, 1),]);
    }

    #[test]
    fn test_iter_1x3() {
        let grid = Grid::<bool>::default(1, 3);

        assert_eq!(
            grid.iter().collect::<Vec<_>>(),
            vec![xy(0, 0), xy(0, 1), xy(0, 2),]
        );
    }

    #[test]
    fn test_iter_2x0() {
        let grid = Grid::<bool>::default(2, 0);

        assert_eq!(grid.iter().collect::<Vec<_>>(), vec![]);
    }

    #[test]
    fn test_iter_2x1() {
        let grid = Grid::<bool>::default(2, 1);

        assert_eq!(grid.iter().collect::<Vec<_>>(), vec![xy(0, 0), xy(1, 0),]);
    }

    #[test]
    fn test_iter_2x2() {
        let grid = Grid::<bool>::default(2, 2);

        assert_eq!(
            grid.iter().collect::<Vec<_>>(),
            vec![xy(0, 0), xy(1, 0), xy(0, 1), xy(1, 1),]
        );
    }

    #[test]
    fn test_iter_2x3() {
        let grid = Grid::<bool>::default(2, 3);

        assert_eq!(
            grid.iter().collect::<Vec<_>>(),
            vec![xy(0, 0), xy(1, 0), xy(0, 1), xy(1, 1), xy(0, 2), xy(1, 2),]
        );
    }

    #[test]
    fn test_iter_3x0() {
        let grid = Grid::<bool>::default(3, 0);

        assert_eq!(grid.iter().collect::<Vec<_>>(), vec![]);
    }

    #[test]
    fn test_iter_3x1() {
        let grid = Grid::<bool>::default(3, 1);

        assert_eq!(
            grid.iter().collect::<Vec<_>>(),
            vec![xy(0, 0), xy(1, 0), xy(2, 0),]
        );
    }

    #[test]
    fn test_iter_3x2() {
        let grid = Grid::<bool>::default(3, 2);

        assert_eq!(
            grid.iter().collect::<Vec<_>>(),
            vec![xy(0, 0), xy(1, 0), xy(2, 0), xy(0, 1), xy(1, 1), xy(2, 1),]
        );
    }

    #[test]
    fn test_iter_3x3() {
        let grid = Grid::<bool>::default(3, 3);

        assert_eq!(
            grid.iter().collect::<Vec<_>>(),
            vec![
                xy(0, 0),
                xy(1, 0),
                xy(2, 0),
                xy(0, 1),
                xy(1, 1),
                xy(2, 1),
                xy(0, 2),
                xy(1, 2),
                xy(2, 2),
            ]
        );
    }

    #[test]
    fn test_add() {
        let a = Grid::from_element(2, 3, 4);
        let b = Grid::from_element(2, 3, 5);

        assert_eq!(a + b, Grid::from_element(2, 3, 9));
    }

    #[test]
    #[should_panic(expected = "Trying to add grid with dimensions 2x3 to grid with dimensions 4x5")]
    fn test_add_dimension_mismatch() {
        let a = Grid::from_element(2, 3, 4);
        let b = Grid::from_element(4, 5, 5);

        let _ = a + b;
    }

    #[test]
    fn test_sub() {
        let a = Grid::from_element(2, 3, 5);
        let b = Grid::from_element(2, 3, 4);

        assert_eq!(a - b, Grid::from_element(2, 3, 1));
    }

    #[test]
    #[should_panic(
        expected = "Trying to subtract grid with dimensions 4x5 from grid with dimensions 2x3"
    )]
    fn test_sub_dimension_mismatch() {
        let a = Grid::from_element(2, 3, 5);
        let b = Grid::from_element(4, 5, 4);

        let _ = a - b;
    }
}
