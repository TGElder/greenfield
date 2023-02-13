use commons::geometry::Rectangle;

pub trait Projection {
    fn projection(&self) -> &[[f32; 4]; 4];
    fn unproject(&self, gl_xyz: &[f32; 3]) -> [f32; 3];
    fn look_at(&mut self, world_xyz: &[f32; 3], screen_xy: &[f32; 2]);
    fn yaw(&mut self, yaw: f32);
    fn zoom(&mut self, zoom: f32);
    fn set_viewport(&mut self, viewport: Rectangle<u32>);
}
