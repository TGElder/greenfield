use std::borrow::Borrow;
use std::ops::{Index, IndexMut};

#[derive(Debug)]
struct Grid<T> {
    width: usize,
    height: usize,
    elements: Vec<T>,
}

impl<T> Grid<T> {
    pub fn from_vec(width: usize, height: usize, elements: Vec<T>) -> Grid<T> {
        let len = width * height;
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

    pub fn from_fn<F>(width: usize, height: usize, mut function: F) -> Grid<T>
    where
        F: FnMut((usize, usize)) -> T,
    {
        let mut elements = Vec::with_capacity(width * height);
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
        R: Borrow<(usize, usize)>,
    {
        let (x, y) = xy.borrow();
        x + y * self.width
    }

    pub fn xy<R>(&self, index: R) -> (usize, usize)
    where
        R: Borrow<usize>,
    {
        let index = index.borrow();
        (index % self.width, index / self.width)
    }

    pub fn in_bounds<R>(&self, xy: R) -> bool
    where
        R: Borrow<(usize, usize)>,
    {
        let (x, y) = xy.borrow();
        *x < self.width && *y < self.height
    }

    pub fn for_each<F>(&self, mut function: F)
    where
        F: FnMut((usize, usize), &T),
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
        F: FnMut((usize, usize), &T) -> U,
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
}

impl<T> Grid<T>
where
    T: Default,
{
    pub fn new(width: usize, height: usize) -> Grid<T> {
        Grid {
            elements: (0..width * height).map(|_| T::default()).collect(),
            width,
            height,
        }
    }
}

impl<T> Grid<T>
where
    T: Clone,
{
    pub fn from_element(width: usize, height: usize, element: T) -> Grid<T> {
        Grid {
            elements: vec![element; width * height],
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

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.elements[self.index(index)]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let index = self.index(index);
        &mut self.elements[index]
    }
}

struct ForEachIterator<'a, T> {
    x: usize,
    y: usize,
    grid: &'a Grid<T>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let grid: Grid<bool> = Grid::new(4, 5);

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
    #[should_panic(expected = "Number of elements 1 does not equal (width=2 * height=3 = 6)")]
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
}
