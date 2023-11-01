use std::borrow::Borrow;
use std::f32::consts::PI;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

use num::{Float, One};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct XY<T> {
    pub x: T,
    pub y: T,
}

impl<T> XY<T>
where
    T: Float + From<f32>,
{
    pub fn magnitude(&self) -> T {
        (self.x.powf(2.0.into()) + self.y.powf(2.0.into())).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        xy(self.x.div(magnitude), self.y.div(magnitude))
    }

    pub fn angle(&self) -> T {
        let XY { x, y } = self.normalize();
        let angle = y.atan2(x);
        if angle < T::zero() {
            angle.add((2.0 * PI).into())
        } else {
            angle
        }
    }
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

impl <T> XYZ<T> where T: Copy {
    pub fn xy(&self) -> XY<T> {
        XY{
            x: self.x,
            y: self.y
        }
    }    
}

impl<T> XYZ<T>
where
    T: Float + From<f32>,
{
    pub fn magnitude(&self) -> T {
        (self.x.powf(2.0.into()) + self.y.powf(2.0.into()) + self.z.powf(2.0.into())).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        xyz(
            self.x.div(magnitude),
            self.y.div(magnitude),
            self.z.div(magnitude),
        )
    }
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
pub struct XYRectangle<T> {
    pub from: XY<T>,
    pub to: XY<T>,
}

impl<T> XYRectangle<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + One,
{
    pub fn width(&self) -> T {
        self.to.x.sub(self.from.x) + T::one()
    }

    pub fn height(&self) -> T {
        self.to.y.sub(self.from.y) + T::one()
    }
}

impl<T> XYRectangle<T>
where
    T: PartialOrd,
{
    pub fn contains<B>(&self, position: B) -> bool
    where
        B: Borrow<XY<T>>,
    {
        let position = position.borrow();
        !(position.x < self.from.x
            || position.x > self.to.x
            || position.y < self.from.y
            || position.y > self.to.y)
    }
}

#[cfg(test)]
mod tests {
    use crate::almost_eq::assert_almost_eq;

    use super::*;

    #[test]
    fn test_xy_magnitude() {
        // given
        let a = xy(3.0, 4.0);

        // when
        let result = a.magnitude();

        // then
        assert_almost_eq(result, 5.0);
    }

    #[test]
    fn test_xy_normalize() {
        // given
        let a = xy(3.0, 4.0);

        // when
        let result = a.normalize();

        // then
        assert_almost_eq(result.x, 3.0 / 5.0);
        assert_almost_eq(result.y, 4.0 / 5.0);
    }

    #[test]
    fn test_angle() {
        assert_almost_eq(xy(1.0, 0.0).angle(), 0.0);
        assert_almost_eq(xy(1.0, 1.0).angle(), 1.0 * PI / 4.0);
        assert_almost_eq(xy(0.0, 1.0).angle(), 2.0 * PI / 4.0);
        assert_almost_eq(xy(-1.0, 1.0).angle(), 3.0 * PI / 4.0);
        assert_almost_eq(xy(-1.0, 0.0).angle(), 4.0 * PI / 4.0);
        assert_almost_eq(xy(-1.0, -1.0).angle(), 5.0 * PI / 4.0);
        assert_almost_eq(xy(0.0, -1.0).angle(), 6.0 * PI / 4.0);
        assert_almost_eq(xy(1.0, -1.0).angle(), 7.0 * PI / 4.0);
    }

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
    fn test_xyz_xy() {
         // given
         let a = xyz(3, 4, 5);

         // when
         let result = a.xy();

         // then
         assert_eq!(result, xy(3, 4));
  
    }

    #[test]
    fn test_xyz_magnitude() {
        // given
        let a = xyz(3.0, 4.0, 5.0);

        // when
        let result = a.magnitude();

        // then
        assert_almost_eq(result, 50.0.sqrt());
    }

    #[test]
    fn test_xyz_normalize() {
        // given
        let a = xyz(3.0, 4.0, 5.0);

        // when
        let result = a.normalize();

        // then
        assert_almost_eq(result.x, 3.0 / 50.0.sqrt());
        assert_almost_eq(result.y, 4.0 / 50.0.sqrt());
        assert_almost_eq(result.z, 5.0 / 50.0.sqrt());
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

    #[test]
    fn test_xy_rectangle_width() {
        // given
        let rectangle = XYRectangle {
            from: xy(1, 2),
            to: xy(3, 5),
        };

        // when
        let result = rectangle.width();

        // then
        assert_eq!(result, 3);
    }

    #[test]
    fn test_xy_rectangle_height() {
        // given
        let rectangle = XYRectangle {
            from: xy(1, 2),
            to: xy(3, 5),
        };

        // when
        let result = rectangle.height();

        // then
        assert_eq!(result, 4);
    }

    #[test]
    fn test_xy_rectangle_contains() {
        // given
        let rectangle = XYRectangle {
            from: xy(1, 2),
            to: xy(3, 5),
        };

        // then
        assert!(rectangle.contains(xy(2, 3)));
        assert!(!rectangle.contains(xy(0, 3)));
        assert!(!rectangle.contains(xy(4, 3)));
        assert!(!rectangle.contains(xy(2, 1)));
        assert!(!rectangle.contains(xy(2, 6)));
    }
}
