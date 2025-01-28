use std::borrow::Borrow;
use std::f32::consts::PI;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

use num::{Float, One, Saturating};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
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

impl<T> XY<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T>,
{
    pub fn dot(&self, rhs: &XY<T>) -> T {
        self.x * rhs.x + self.y * rhs.y
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

impl<T> PartialOrd<XY<T>> for XY<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &XY<T>) -> Option<std::cmp::Ordering> {
        Some(
            self.x
                .partial_cmp(&other.x)?
                .then(self.y.partial_cmp(&other.y)?),
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct XYZ<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> XYZ<T>
where
    T: Copy,
{
    pub fn xy(&self) -> XY<T> {
        XY {
            x: self.x,
            y: self.y,
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

impl<T> XYZ<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T>,
{
    pub fn dot(&self, rhs: &XYZ<T>) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
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

#[derive(Debug)]
pub struct Line<T> {
    pub from: XY<T>,
    pub to: XY<T>,
}

pub fn project_point_onto_line<T>(point: XY<T>, line: Line<T>) -> Result<XY<T>, &'static str>
where
    T: Float + From<f32>,
{
    if line.from == line.to {
        return Err("Cannot project point onto zero length line");
    }
    if line.from == point {
        return Ok(line.from);
    }
    let from2to = line.to - line.from;
    let from2point = point - line.from;
    let dot_product = from2to.dot(&from2point);
    let cos = dot_product / (from2to.magnitude() * from2point.magnitude());
    let from2target_magnitude = cos * from2point.magnitude();
    let from2to_ratio = from2target_magnitude / from2to.magnitude();
    Ok(line.from + from2to * from2to_ratio)
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Rectangle<T> {
    pub width: T,
    pub height: T,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
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

impl<T> XYRectangle<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Saturating,
{
    pub fn expand(&self, cell_count: T) -> XYRectangle<T> {
        XYRectangle {
            from: xy(
                self.from.x.saturating_sub(cell_count),
                self.from.y.saturating_sub(cell_count),
            ),
            to: xy(
                self.to.x.saturating_add(cell_count),
                self.to.y.saturating_add(cell_count),
            ),
        }
    }
}

impl XYRectangle<u32> {
    pub fn iter(&self) -> impl Iterator<Item = XY<u32>> + '_ {
        (self.from.x..=self.to.x)
            .flat_map(move |x| (self.from.y..=self.to.y).map(move |y| XY { x, y }))
    }
}

impl XYRectangle<u64> {
    pub fn iter(&self) -> impl Iterator<Item = XY<u64>> + '_ {
        (self.from.x..=self.to.x)
            .flat_map(move |x| (self.from.y..=self.to.y).map(move |y| XY { x, y }))
    }
}

impl XYRectangle<usize> {
    pub fn iter(&self) -> impl Iterator<Item = XY<usize>> + '_ {
        (self.from.x..=self.to.x)
            .flat_map(move |x| (self.from.y..=self.to.y).map(move |y| XY { x, y }))
    }
}

impl<T> XYRectangle<T> {
    pub fn overlaps<B>(&self, other: B) -> bool
    where
        T: PartialOrd,
        B: Borrow<XYRectangle<T>>,
    {
        let other = other.borrow();
        (self.from.x <= other.to.x && self.to.x >= other.from.x)
            && (self.from.y <= other.to.y && self.to.y >= other.from.y)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

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
    fn test_xy_dot() {
        // given
        let a = xy(2, 4);
        let b = xy(3, 5);

        // when
        let result = a.dot(&b);

        // then
        assert_eq!(result, 26);
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
    fn test_xyz_dot() {
        // given
        let a = xyz(2, 4, 6);
        let b = xyz(3, 5, 7);

        // when
        let result = a.dot(&b);

        // then
        assert_eq!(result, 68);
    }

    #[test]
    fn test_project_point_onto_line() {
        // given
        let point = xy(3.0, 1.0);
        let line = Line {
            from: xy(2.0, 1.0),
            to: xy(6.0, 2.0),
        };

        // when
        let result = project_point_onto_line(point, line).unwrap();

        // then
        assert_almost_eq(result.x, 2.9411764705882355);
        assert_almost_eq(result.y, 1.2352941176470589);
    }

    #[test]
    fn test_project_point_onto_line_from_equals_to() {
        // given
        let point = xy(3.0, 1.0);
        let line = Line {
            from: xy(2.0, 1.0),
            to: xy(2.0, 1.0),
        };

        // when
        let result = project_point_onto_line(point, line);

        // then
        assert_eq!(result, Err("Cannot project point onto zero length line"));
    }

    #[test]
    fn test_project_point_onto_line_point_equals_from() {
        // given
        let point = xy(2.0, 1.0);
        let line = Line {
            from: xy(2.0, 1.0),
            to: xy(6.0, 2.0),
        };

        // when
        let result = project_point_onto_line(point, line).unwrap();

        // then
        assert_almost_eq(result.x, 2.0);
        assert_almost_eq(result.y, 1.0);
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

    #[test]
    fn test_xy_rectangle_expand() {
        // given
        let rectangle: XYRectangle<u32> = XYRectangle {
            from: xy(1, 2),
            to: xy(3, 5),
        };

        // then
        assert_eq!(
            rectangle.expand(2),
            XYRectangle {
                from: xy(0, 0),
                to: xy(5, 7),
            }
        );
    }

    #[test]
    fn test_xy_rectangle_iter_u32() {
        // given
        let rectangle = XYRectangle {
            from: xy(1u32, 2),
            to: xy(3, 5),
        };

        // when
        let result = rectangle.iter().collect::<HashSet<_>>();

        // then
        assert_eq!(
            result,
            HashSet::from([
                xy(1, 2),
                xy(1, 3),
                xy(1, 4),
                xy(1, 5),
                xy(2, 2),
                xy(2, 3),
                xy(2, 4),
                xy(2, 5),
                xy(3, 2),
                xy(3, 3),
                xy(3, 4),
                xy(3, 5),
            ])
        );
    }

    #[test]
    fn test_xy_rectangle_iter_u64() {
        // given
        let rectangle = XYRectangle {
            from: xy(1u64, 2),
            to: xy(3, 5),
        };

        // when
        let result = rectangle.iter().collect::<HashSet<_>>();

        // then
        assert_eq!(
            result,
            HashSet::from([
                xy(1, 2),
                xy(1, 3),
                xy(1, 4),
                xy(1, 5),
                xy(2, 2),
                xy(2, 3),
                xy(2, 4),
                xy(2, 5),
                xy(3, 2),
                xy(3, 3),
                xy(3, 4),
                xy(3, 5),
            ])
        );
    }

    #[test]
    fn test_xy_rectangle_iter_usize() {
        // given
        let rectangle = XYRectangle {
            from: xy(1usize, 2),
            to: xy(3, 5),
        };

        // when
        let result = rectangle.iter().collect::<HashSet<_>>();

        // then
        assert_eq!(
            result,
            HashSet::from([
                xy(1, 2),
                xy(1, 3),
                xy(1, 4),
                xy(1, 5),
                xy(2, 2),
                xy(2, 3),
                xy(2, 4),
                xy(2, 5),
                xy(3, 2),
                xy(3, 3),
                xy(3, 4),
                xy(3, 5),
            ])
        );
    }

    #[test]
    fn test_overlaps_partially() {
        let rectangle = XYRectangle {
            from: xy(1usize, 2),
            to: xy(4, 5),
        };
        let other = XYRectangle {
            from: xy(3usize, 4),
            to: xy(5, 6),
        };
        assert!(rectangle.overlaps(other));
        assert!(other.overlaps(rectangle));
    }

    #[test]
    fn test_overlaps_rectangle_contains_other_rectangle() {
        let rectangle = XYRectangle {
            from: xy(1usize, 2),
            to: xy(4, 5),
        };
        let other = XYRectangle {
            from: xy(2usize, 3),
            to: xy(3, 4),
        };
        assert!(rectangle.overlaps(other));
        assert!(other.overlaps(rectangle));
    }

    #[test]
    fn test_overlaps_x_only() {
        let rectangle = XYRectangle {
            from: xy(1usize, 2),
            to: xy(4, 5),
        };
        let other = XYRectangle {
            from: xy(2usize, 6),
            to: xy(3, 7),
        };
        assert!(!rectangle.overlaps(other));
        assert!(!other.overlaps(rectangle));
    }

    #[test]
    fn test_overlaps_y_only() {
        let rectangle = XYRectangle {
            from: xy(1usize, 2),
            to: xy(4, 5),
        };
        let other = XYRectangle {
            from: xy(5usize, 3),
            to: xy(6, 4),
        };
        assert!(!rectangle.overlaps(other));
        assert!(!other.overlaps(rectangle));
    }
}
