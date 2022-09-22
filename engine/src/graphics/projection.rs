pub trait Projection {
    fn projection(&self) -> &[[f32; 4]; 4];
}
