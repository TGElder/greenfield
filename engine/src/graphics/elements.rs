use commons::color::Rgb;
use commons::geometry::XYZ;

pub struct Triangle {
    pub corners: [XYZ<f32>; 3],
    pub color: Rgb<f32>,
}

pub struct Quad {
    pub corners: [XYZ<f32>; 4],
    pub color: Rgb<f32>,
}
