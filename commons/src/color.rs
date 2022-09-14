#[derive(Clone, Copy)]
pub struct Color<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T> Color<T> {
    pub fn rgb(r: T, g: T, b: T) -> Color<T> {
        Color { r, g, b }
    }
}
