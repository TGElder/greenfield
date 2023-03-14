use commons::color::Rgb;
use commons::geometry::{Rectangle, XYZ};

#[derive(Clone, Copy)]
pub struct Triangle {
    pub corners: [XYZ<f32>; 3],
    pub color: Rgb<f32>,
}

#[derive(Clone, Copy)]
pub struct Quad {
    pub corners: [XYZ<f32>; 4],
    pub color: Rgb<f32>,
}

#[derive(Clone, Copy)]
pub struct Billboard {
    pub position: XYZ<f32>,
    pub dimensions: Rectangle<f32>,
    pub texture: usize,
}
