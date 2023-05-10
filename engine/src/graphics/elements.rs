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

#[derive(Clone)]
pub struct OverlayTriangles {
    pub base_texture: usize,
    pub overlay_texture: usize,
    pub triangles: Vec<[TexturedPosition; 3]>,
}

#[derive(Clone)]
pub struct OverlayQuads {
    pub base_texture: usize,
    pub overlay_texture: usize,
    pub quads: Vec<[TexturedPosition; 4]>,
}

#[derive(Clone, Copy)]
pub struct TexturedPosition {
    pub position: XYZ<f32>,
    pub texture_coordinates: XY<f32>,
}

#[derive(Clone, Copy)]
pub struct Billboard {
    pub position: XYZ<f32>,
    pub dimensions: Rectangle<f32>,
    pub texture: usize,
}
