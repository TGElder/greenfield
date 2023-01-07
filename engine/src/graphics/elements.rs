use commons::color::Rgb;

pub struct Triangle {
    pub id: u32,
    pub corners: [[f32; 3]; 3],
    pub color: Rgb<f32>,
}

pub struct Quad {
    pub id: u32,
    pub corners: [[f32; 3]; 4],
    pub color: Rgb<f32>,
}
