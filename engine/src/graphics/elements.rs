use commons::color::Rgb;

pub struct Triangle {
    pub corners: [[f32; 3]; 3],
    pub color: Rgb<f32>,
}

pub struct Quad {
    pub corners: [[f32; 3]; 4],
    pub color: Rgb<f32>,
}
