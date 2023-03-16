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
