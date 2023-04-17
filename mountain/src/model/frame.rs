use commons::geometry::XYZ;

#[derive(Clone, Copy, Debug)]
pub struct Frame {
    pub position: XYZ<f32>,
    pub angle: f32,
}
