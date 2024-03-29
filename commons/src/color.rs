#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rgb<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T> Rgb<T> {
    pub const fn new(r: T, g: T, b: T) -> Rgb<T> {
        Rgb { r, g, b }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rgba<T> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}

impl<T> Rgba<T> {
    pub const fn new(r: T, g: T, b: T, a: T) -> Rgba<T> {
        Rgba { r, g, b, a }
    }
}
