use commons::color::Rgb;
use commons::geometry::{Rectangle, XY, XYZ};

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

#[derive(Clone)]
pub struct TriangleOverlay {
    pub triangles: Vec<[TexturedPosition; 3]>,
    pub base_texture: usize,
    pub overlay_texture: usize,
}

#[derive(Clone)]
pub struct QuadOverlay {
    pub quads: Vec<[TexturedPosition; 4]>,
    pub base_texture: usize,
    pub overlay_texture: usize,
}

#[derive(Clone, Copy)]
pub struct TexturedPosition {
    pub position: XYZ<f32>,
    pub texture_coordinates: XY<f32>,
}
