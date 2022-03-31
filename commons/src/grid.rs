use std::borrow::Borrow;
use std::fmt::Display;
use std::ops::{Add, Div, Index, IndexMut, Mul, Range, Rem};

#[derive(Debug)]
struct Grid<T, U> {
    width: T,
    height: T,
    elements: Vec<U>,
}

impl<T, U> Grid<T, U>
where
    T: Add + Copy + Default + Display + Div + Mul + Into<usize> + Rem + PartialOrd,
    Range<T>: Iterator<Item = T>,
{
    pub fn from_vec(width: T, height: T, elements: Vec<U>) -> Grid<T, U> {
        let len = width.into() * height.into();
        if elements.len() != len {
            panic!(
                "Number of elements {} does not equal (width={} * height={} = {})",
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

    pub fn from_fn<F>(width: T, height: T, mut function: F) -> Grid<T, U>
    where
        F: FnMut((T, T)) -> U,
    {
        let mut elements = Vec::with_capacity(width.into() * height.into());
        for y in T::default()..height {
            for x in T::default()..width {
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
        R: Borrow<(T, T)>,
    {
        let (x, y) = xy.borrow();
        (*x).into() + (*y).into() * self.width.into()
    }

    pub fn xy<R>(&self, index: R) -> (usize, usize)
    where
        R: Borrow<usize>,
    {
        let index = index.borrow();
        (index % self.width.into(), index / self.width.into())
    }

    pub fn in_bounds<R>(&self, xy: R) -> bool
    where
        R: Borrow<(T, T)>,
    {
        let (x, y) = xy.borrow();
        *x < self.width && *y < self.height
    }

    pub fn for_each<F>(&self, mut function: F)
    where
        F: FnMut((T, T), &U),
    {
        let mut index = 0;
        for y in T::default()..self.height {
            for x in T::default()..self.width {
                function((x, y), &self.elements[index]);
                index += 1;
            }
        }
    }

    pub fn map<F, V>(&self, mut function: F) -> Grid<T, V>
    where
        F: FnMut((T, T), &U) -> V,
    {
        let mut elements = Vec::with_capacity(self.elements.len());
        let mut index = 0;
        for y in T::default()..self.height {
            for x in T::default()..self.width {
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
}

impl<T, U> Grid<T, U>
where
    T: Copy + Default + Into<usize>,
    Range<T>: Iterator<Item = T>,
    U: Default,
{
    pub fn new(width: T, height: T) -> Grid<T, U> {
        Grid {
            elements: (0..width.into() * height.into())
                .map(|_| U::default())
                .collect(),
            width,
            height,
        }
    }
}

impl<T, U> Grid<T, U>
where
    T: Copy + Into<usize>,
    U: Clone,
{
    pub fn from_element(width: T, height: T, element: U) -> Grid<T, U> {
        Grid {
            elements: vec![element; width.into() * height.into()],
            width,
            height,
        }
    }
}

impl<T, U> PartialEq for Grid<T, U>
where
    T: PartialEq,
    U: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height && self.elements == other.elements
    }
}

impl<T, U> Index<(T, T)> for Grid<T, U>
where
    T: Add + Copy + Default + Display + Div + Mul + Into<usize> + Rem + PartialOrd,
    Range<T>: Iterator<Item = T>,
{
    type Output = U;

    fn index(&self, index: (T, T)) -> &Self::Output {
        &self.elements[self.index(index)]
    }
}

impl<T, U> IndexMut<(T, T)> for Grid<T, U>
where
    T: Add + Copy + Default + Display + Div + Mul + Into<usize> + Rem + PartialOrd,
    Range<T>: Iterator<Item = T>,
{
    fn index_mut(&mut self, index: (T, T)) -> &mut Self::Output {
        let index = self.index(index);
        &mut self.elements[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let grid: Grid<u8, bool> = Grid::new(4, 5);

        assert_eq!(
            grid,
            Grid {
                width: 4,
                height: 5,
                elements: vec![false; 20],
            }
        );
    }

    #[test]
    fn test_from_element() {
        let grid: Grid<u8, Option<u8>> = Grid::from_element(4, 5, Some(3));

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
    fn test_from_vec() {
        let grid: Grid<u8, (u8, u8)> = Grid::from_vec(
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
    #[should_panic(expected = "Number of elements 1 does not equal (width=2 * height=3 = 6)")]
    fn test_from_vec_of_wrong_length() {
        Grid::<u8, bool>::from_vec(2, 3, vec![false]);
    }

    #[test]
    fn test_from_fn() {
        let grid = Grid::<u8, (u8, u8)>::from_fn(2, 3, |t| t);

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
    fn test_index() {
        let grid = Grid::<u8, bool>::from_element(2, 3, false);

        assert_eq!(grid.index((0, 0)), 0);
        assert_eq!(grid.index((1, 0)), 1);
        assert_eq!(grid.index((0, 1)), 2);
        assert_eq!(grid.index((1, 1)), 3);
        assert_eq!(grid.index((0, 2)), 4);
        assert_eq!(grid.index((1, 2)), 5);
    }

    #[test]
    fn test_xy() {
        let grid = Grid::<u8, bool>::from_element(2, 3, false);

        assert_eq!(grid.xy(0), (0, 0));
        assert_eq!(grid.xy(1), (1, 0));
        assert_eq!(grid.xy(2), (0, 1));
        assert_eq!(grid.xy(3), (1, 1));
        assert_eq!(grid.xy(4), (0, 2));
        assert_eq!(grid.xy(5), (1, 2));
    }

    #[test]
    fn test_in_bounds() {
        let grid = Grid::<u8, bool>::from_element(2, 3, false);

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
        let grid = Grid::<u8, u8>::from_element(2, 3, 1);

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
        let grid = Grid::<u8, u8>::from_element(2, 3, 1);

        // when
        let mut acc = 0;
        grid.for_each(|(x, y), z| acc += x + y + z);

        // then
        assert_eq!(acc, 15);
    }
}
