use commons::geometry::{Rectangle, XY, XYZ};

#[derive(Clone, Copy, Debug)]
pub struct Triangle<T> {
    pub corners: [XYZ<f32>; 3],
    pub normal: XYZ<f32>,
    pub color: T,
}

#[derive(Clone, Copy, Debug)]
pub struct Quad<T> {
    pub corners: [XYZ<f32>; 4],
    pub color: T,
}

#[derive(Clone, Debug)]
pub struct OverlayTriangles {
    pub base_texture: usize,
    pub overlay_texture: usize,
    pub triangles: Vec<[TexturedPosition; 3]>,
}

#[derive(Clone, Copy, Debug)]
pub struct TexturedPosition {
    pub position: XYZ<f32>,
    pub normal: XYZ<f32>,
    pub texture_coordinates: XY<f32>,
}

#[derive(Clone, Copy, Debug)]
pub struct Billboard {
    pub position: XYZ<f32>,
    pub dimensions: Rectangle<f32>,
    pub texture: usize,
}
