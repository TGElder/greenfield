#[derive(Copy, Clone)]
pub struct ColoredVertex {
    pub id: u32,
    pub position: [f32; 3],
    pub color: [f32; 3],
}
glium::implement_vertex!(ColoredVertex, position, id);

#[derive(Copy, Clone)]
pub struct ScreenVertex {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
}
glium::implement_vertex!(ScreenVertex, position, tex_coords);
