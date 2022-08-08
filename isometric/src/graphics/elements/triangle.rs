use commons::color::Color;

pub struct Triangle {
    pub id: u32,
    pub corners: [[f32; 3]; 3],
    pub color: Color<f32>,
}
