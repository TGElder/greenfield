use commons::geometry::{Rectangle, XY, XYZ};

pub trait Projection {
    fn projection(&self) -> &[[f32; 4]; 4];
    fn unproject(&self, gl_xyz: &XYZ<f32>) -> XYZ<f32>;
    fn look_at(&mut self, world_xyz: &XYZ<f32>, screen_xy: &XY<f32>);
    fn yaw(&mut self, yaw: f32);
    fn zoom(&mut self, zoom: f32);
    fn set_viewport(&mut self, viewport: Rectangle<u32>);
}
