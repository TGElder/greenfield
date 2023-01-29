pub trait Projection {
    fn projection(&self) -> &[[f32; 4]; 4];
    fn look_at(&mut self, world_xyz: &[f32; 3], screen_xy: &[f32; 2]);
}
