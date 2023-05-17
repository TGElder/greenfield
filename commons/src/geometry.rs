use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct XY<T> {
    pub x: T,
    pub y: T,
}

pub const fn xy<T>(x: T, y: T) -> XY<T> {
    XY { x, y }
}

impl<T> Display for XY<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {})", self.x, self.y)
    }
}

impl<T> From<XY<T>> for [T; 2] {
    fn from(position: XY<T>) -> Self {
        [position.x, position.y]
    }
}

impl<T> Add for XY<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = XY<T>;

    fn add(self, rhs: Self) -> Self::Output {
        xy(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> Sub for XY<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = XY<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        xy(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> Mul<T> for XY<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        xy(self.x * rhs, self.y * rhs)
    }
}

impl<T> Div<T> for XY<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        xy(self.x / rhs, self.y / rhs)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct XYZ<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub const fn xyz<T>(x: T, y: T, z: T) -> XYZ<T> {
    XYZ { x, y, z }
}

impl<T> Display for XYZ<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.x, self.y, self.z)
    }
}

impl<T> From<XYZ<T>> for [T; 3] {
    fn from(position: XYZ<T>) -> Self {
        [position.x, position.y, position.z]
    }
}

impl<T> Add for XYZ<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = XYZ<T>;

    fn add(self, rhs: Self) -> Self::Output {
        xyz(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T> Sub for XYZ<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = XYZ<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        xyz(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T> Mul<T> for XYZ<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        xyz(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl<T> Div<T> for XYZ<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        xyz(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rectangle<T> {
    pub width: T,
    pub height: T,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PositionedRectangle<T> {
    pub from: XY<T>,
    pub to: XY<T>,
}

impl<T> PositionedRectangle<T>
where
    T: Copy + Sub<Output = T>,
{
    pub fn width(&self) -> T {
        self.to.x.sub(self.from.x)
    }

    pub fn height(&self) -> T {
        self.to.y.sub(self.from.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xy_add() {
        // given
        let a = xy(1, 2);
        let b = xy(3, 4);

        // when
        let result = a + b;

        // then
        assert_eq!(result, xy(4, 6));
    }

    #[test]
    fn test_xy_sub() {
        // given
        let a = xy(1, 2);
        let b = xy(3, 4);

        // when
        let result = b - a;

        // then
        assert_eq!(result, xy(2, 2));
    }

    #[test]
    fn test_xy_mul() {
        // given
        let a = xy(1, 2);

        // when
        let result = a * 2;

        // then
        assert_eq!(result, xy(2, 4));
    }

    #[test]
    fn test_xy_div() {
        // given
        let a = xy(2, 4);

        // when
        let result = a / 2;

        // then
        assert_eq!(result, xy(1, 2));
    }

    #[test]
    fn test_xyz_add() {
        // given
        let a = xyz(1, 2, 3);
        let b = xyz(4, 5, 6);

        // when
        let result = a + b;

        // then
        assert_eq!(result, xyz(5, 7, 9));
    }

    #[test]
    fn test_xyz_sub() {
        // given
        let a = xyz(1, 2, 3);
        let b = xyz(4, 5, 6);

        // when
        let result = b - a;

        // then
        assert_eq!(result, xyz(3, 3, 3));
    }

    #[test]
    fn test_xyz_mul() {
        // given
        let a = xyz(1, 2, 3);

        // when
        let result = a * 2;

        // then
        assert_eq!(result, xyz(2, 4, 6));
    }

    #[test]
    fn test_xyz_div() {
        // given
        let a = xyz(2, 4, 6);

        // when
        let result = a / 2;

        // then
        assert_eq!(result, xyz(1, 2, 3));
    }
}
