use commons::geometry::XYZ;

#[derive(Clone, Copy, Debug)]
pub struct Frame {
    pub position: XYZ<f32>,
    pub yaw: f32,
    pub pitch: f32,
    pub model_offset: Option<XYZ<f32>>,
    pub model: Model,
}

#[derive(Clone, Copy, Debug)]
pub enum Model {
    Standing { skis: bool },
    Sitting,
    Chair,
}
