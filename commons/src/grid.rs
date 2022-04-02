use std::borrow::Borrow;
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Grid<T> {
    width: u32,
    height: u32,
    elements: Vec<T>,
}

impl<T> Grid<T> {
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
        F: FnMut((u32, u32)) -> T,
    {
        let mut elements = Vec::with_capacity(width as usize * height as usize);
        for y in 0..height {
            for x in 0..width {
                elements.push(function((x, y)));
            }
        }
        Grid {
            width,
            height,
            elements,
        }
    }

    pub fn index<R>(&self, xy: R) -> usize
    where
        R: Borrow<(u32, u32)>,
    {
        let (x, y) = xy.borrow();
        (*x as usize) + (*y as usize) * self.width as usize
    }

    pub fn xy<R>(&self, index: R) -> (usize, usize)
    where
        R: Borrow<usize>,
    {
        let index = index.borrow();
        (index % self.width as usize, index / self.width as usize)
    }

    pub fn in_bounds<R>(&self, xy: R) -> bool
    where
        R: Borrow<(u32, u32)>,
    {
        let (x, y) = xy.borrow();
        *x < self.width && *y < self.height
    }

    pub fn for_each<F>(&self, mut function: F)
    where
        F: FnMut((u32, u32), &T),
    {
        let mut index = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                function((x, y), &self.elements[index]);
                index += 1;
            }
        }
    }

    pub fn map<F, U>(&self, mut function: F) -> Grid<U>
    where
        F: FnMut((u32, u32), &T) -> U,
    {
        let mut elements = Vec::with_capacity(self.elements.len());
        let mut index = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                elements.push(function((x, y), &self.elements[index]));
                index += 1;
            }
        }
        Grid {
            width: self.width,
            height: self.height,
            elements,
        }
    }

    pub fn offset<R, S>(&self, xy: R, offset: S) -> Option<(u32, u32)>
    where
        R: Borrow<(u32, u32)>,
        S: Borrow<(i32, i32)>,
    {
        let (x, y) = xy.borrow();
        let (dx, dy) = offset.borrow();

        let nx = (*x) as i64 + (*dx) as i64;
        let ny = (*y) as i64 + (*dy) as i64;

        let nx = nx.try_into().ok()?;
        let ny = ny.try_into().ok()?;

        if !self.in_bounds((nx, ny)) {
            return None;
        }

        Some((nx, ny))
    }

    pub fn offsets<'a>(
        &'a self,
        xy: (u32, u32),
        offsets: &'a [(i32, i32)],
    ) -> impl Iterator<Item = (u32, u32)> + 'a {
        offsets.iter().flat_map(move |o| self.offset(xy, o))
    }

    pub fn neighbours_4(&self, xy: (u32, u32)) -> impl Iterator<Item = (u32, u32)> + '_ {
        const OFFSETS_4: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        self.offsets(xy, &OFFSETS_4)
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

impl<T> Index<(u32, u32)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (u32, u32)) -> &Self::Output {
        &self.elements[self.index(index)]
    }
}

impl<T> IndexMut<(u32, u32)> for Grid<T> {
    fn index_mut(&mut self, index: (u32, u32)) -> &mut Self::Output {
        let index = self.index(index);
        &mut self.elements[index]
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use maplit::hashset;

    use super::*;

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

    // Too slow
    // #[test]
    // fn test_from_fn_max_u32() {
    //     Grid::from_fn(u32::MAX, 2, |_| false);
    // }

    #[test]
    fn test_index() {
        let grid = Grid::from_element(2, 3, false);

        assert_eq!(grid.index((0, 0)), 0);
        assert_eq!(grid.index((1, 0)), 1);
        assert_eq!(grid.index((0, 1)), 2);
        assert_eq!(grid.index((1, 1)), 3);
        assert_eq!(grid.index((0, 2)), 4);
        assert_eq!(grid.index((1, 2)), 5);
    }

    #[test]
    fn test_xy() {
        let grid = Grid::from_element(2, 3, false);

        assert_eq!(grid.xy(0), (0, 0));
        assert_eq!(grid.xy(1), (1, 0));
        assert_eq!(grid.xy(2), (0, 1));
        assert_eq!(grid.xy(3), (1, 1));
        assert_eq!(grid.xy(4), (0, 2));
        assert_eq!(grid.xy(5), (1, 2));
    }

    #[test]
    fn test_in_bounds() {
        let grid = Grid::from_element(2, 3, false);

        assert!(grid.in_bounds((0, 0)));
        assert!(grid.in_bounds((1, 0)));
        assert!(!grid.in_bounds((2, 0)));
        assert!(grid.in_bounds((0, 1)));
        assert!(grid.in_bounds((1, 1)));
        assert!(!grid.in_bounds((2, 1)));
        assert!(grid.in_bounds((0, 2)));
        assert!(grid.in_bounds((1, 2)));
        assert!(!grid.in_bounds((2, 2)));
        assert!(!grid.in_bounds((0, 3)));
        assert!(!grid.in_bounds((1, 3)));
        assert!(!grid.in_bounds((2, 3)));
    }

    #[test]
    fn test_map() {
        let grid = Grid::from_element(2, 3, 1);

        assert_eq!(
            grid.map(|(x, y), z| x + y + z),
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
    fn test_for_each() {
        // given
        let grid = Grid::from_element(2, 3, 1);

        // when
        let mut acc = 0;
        grid.for_each(|(x, y), z| acc += x + y + z);

        // then
        assert_eq!(acc, 15);
    }

    #[test]
    fn test_offset() {
        let grid = Grid::from_element(2, 3, false);

        assert_eq!(grid.offset((1, 1), (-1, 1)), Some((0, 2)));
    }

    #[test]
    fn test_offset_out_of_bounds_negative() {
        let grid = Grid::from_element(2, 3, false);

        assert_eq!(grid.offset((0, 1), (-1, 1)), None);
    }

    #[test]
    fn test_offset_out_of_bounds_positive() {
        let grid = Grid::from_element(2, 3, false);

        assert_eq!(grid.offset((1, 1), (2, 1)), None);
    }

    #[test]
    fn test_offset_with_max_u32() {
        let grid = Grid::from_element(u32::MAX, 2, false);

        assert_eq!(
            grid.offset((u32::MAX - 2, 0), (1, 1)),
            Some((u32::MAX - 1, 1))
        );
    }

    #[test]
    fn test_offset_with_max_i32() {
        let grid = Grid::from_element(u32::MAX, 1, false);

        assert_eq!(
            grid.offset((u32::MAX - i32::MAX as u32 - 1, 0), (i32::MAX, 0)),
            Some((u32::MAX - 1, 0))
        );
    }

    #[test]
    fn test_offset_with_min_i32() {
        let grid = Grid::from_element(u32::MAX, 1, false);

        assert_eq!(
            grid.offset((i32::MAX as u32 + 1, 0), (i32::MIN, 0)),
            Some((0, 0))
        );
    }

    #[test]
    fn test_offset_overflow() {
        let grid = Grid::from_element(u32::MAX, 2, false);

        assert_eq!(grid.offset((u32::MAX, 0), (i32::MAX, 1)), None);
    }

    #[test]
    fn test_offsets() {
        let grid = Grid::from_element(2, 3, false);

        assert_eq!(
            grid.offsets((0, 1), &[(1, 0), (0, 1), (-1, -1)])
                .collect::<HashSet<_>>(),
            hashset! {(1, 1), (0, 2)}
        );
    }

    #[test]
    fn test_neighbours_4_in_middle() {
        let grid = Grid::from_element(3, 3, false);

        assert_eq!(
            grid.neighbours_4((1, 1)).collect::<HashSet<_>>(),
            hashset! {(2, 1), (1, 2), (0, 1), (1, 0)}
        );
    }

    #[test]
    fn test_neighbours_4_in_corner() {
        let grid = Grid::from_element(3, 3, false);

        assert_eq!(
            grid.neighbours_4((0, 0)).collect::<HashSet<_>>(),
            hashset! {(1, 0), (0, 1)}
        );
    }
}
