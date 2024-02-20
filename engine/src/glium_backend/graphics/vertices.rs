#[derive(Copy, Clone)]
pub struct ColoredVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub color: [f32; 3],
}
glium::implement_vertex!(ColoredVertex, position, normal, color);

#[derive(Copy, Clone)]
pub struct TexturedVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub texture_coordinates: [f32; 2],
}
glium::implement_vertex!(TexturedVertex, position, normal, texture_coordinates);

#[derive(Copy, Clone)]
pub struct ScreenVertex {
    pub screen_position: [f32; 2],
    pub canvas_position: [f32; 2],
}
glium::implement_vertex!(ScreenVertex, screen_position, canvas_position);

#[derive(Copy, Clone)]
pub struct BillboardVertex {
    pub position: [f32; 3],
    pub offset: [f32; 2],
    pub texture_coordinates: [f32; 2],
}
glium::implement_vertex!(BillboardVertex, position, offset, texture_coordinates);

#[derive(Copy, Clone)]
pub struct InstanceVertex {
    pub world_matrix: [[f32; 4]; 4],
}
glium::implement_vertex!(InstanceVertex, world_matrix);
