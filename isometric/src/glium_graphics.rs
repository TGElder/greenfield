use std::f32::consts::PI;

use crate::graphics::elements::Triangle;
use crate::graphics::isometric::isometric_projection;
use glium::backend::Facade;
use glium::framebuffer::{DepthRenderBuffer, SimpleFrameBuffer};
use glium::index::{NoIndices, PrimitiveType};
use glium::{
    implement_vertex, uniform, Display, DrawParameters, Program, Surface, Texture2d, VertexBuffer,
};
use nalgebra::Matrix4;

pub struct Graphics {
    display: Display,
    matrices: Matrices,
    canvas: Option<Canvas>,
    screen_vertices: VertexBuffer<ScreenVertex>,
    quads: Vec<Option<GliumQuads>>,
    quad_ids: Vec<usize>,
    programs: Programs,
    draw_parameters: DrawParameters<'static>,
}

impl Graphics {
    pub fn new(display: Display) -> Graphics {
        Graphics {
            matrices: Matrices::new(PI / 4.0, 5.0 * PI / 8.0),
            canvas: None,
            screen_vertices: VertexBuffer::new(&display, &SCREEN_QUAD).unwrap(),
            quads: vec![],
            quad_ids: vec![],
            programs: Programs::new(&display),
            display,
            draw_parameters: glium::DrawParameters {
                depth: glium::Depth {
                    test: glium::DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}

struct Matrices {
    pitch: f32,
    yaw: f32,
    projection: Matrix4<f32>,
    scale: Matrix4<f32>,
    translation: Matrix4<f32>,
    composite: Matrix4<f32>,
}

impl Matrices {
    fn new(pitch: f32, yaw: f32) -> Matrices {
        let projection = isometric_projection(&yaw, &pitch);
        let mut scale = Matrix4::new(
            1.0 / 256.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0 / 256.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0 / 256.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        );
        Matrices {
            pitch,
            yaw,
            projection,
            scale: Matrix4::identity(),
            translation: Matrix4::identity(),
            composite: projection * scale,
        }
    }
}

struct Programs {
    screen: Program,
    primitive: Program,
}

impl Programs {
    fn new<F>(display: &F) -> Programs
    where
        F: Facade,
    {
        let vertex_shader_src = r#"
            #version 330

            in vec3 position;
            in uint id;
            out float height;
            flat out float id_in_float;
            flat out int selected;

            uniform uint selection;
            uniform mat4 matrix;

            void main() {
                height = position.z / 32.0;
                id_in_float = uintBitsToFloat(id);
                if (id == selection) {
                    selected = 1;
                } else {
                    selected = 0;
                }
                gl_Position = matrix * vec4(position.x, position.y, position.z, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 330

            in float height;
            flat in float id_in_float;
            flat in int selected;
            out vec4 color;

            void main() {
                if (selected == 1) {
                    color = vec4(1.0, 0.0, 0.0, id_in_float);
                } else {
                    color = vec4(height, height, height, id_in_float);
                }
            }
        "#;

        let screen_vertex_shader_src = r#"
            #version 330

            in vec2 position;
            in vec2 tex_coords;
            out vec2 v_tex_coords;
                            
            void main() {
                v_tex_coords = tex_coords;
                gl_Position = vec4(position, 0.0, 1.0);
            }
        "#;

        let fragment_vertex_shader_src = r#"
            #version 330

            in vec2 v_tex_coords;
            out vec4 color;
            
            uniform sampler2D tex;
            
            void main() {
                color = texture(tex, v_tex_coords);
            }
        "#;

        Programs {
            primitive: Program::from_source(display, vertex_shader_src, fragment_shader_src, None)
                .unwrap(),
            screen: Program::from_source(
                display,
                screen_vertex_shader_src,
                fragment_vertex_shader_src,
                None,
            )
            .unwrap(),
        }
    }
}

struct Canvas {
    width: u32,
    height: u32,
    texture: Texture2d,
    depth_buffer: DepthRenderBuffer,
}

impl Canvas {
    fn new(display: &Display, &(width, height): &(u32, u32)) -> Canvas {
        Canvas {
            width,
            height,
            texture: glium::texture::Texture2d::empty_with_format(
                display,
                glium::texture::UncompressedFloatFormat::F32F32F32F32,
                glium::texture::MipmapsOption::NoMipmap,
                width,
                height,
            )
            .unwrap(),
            depth_buffer: glium::framebuffer::DepthRenderBuffer::new(
                display,
                glium::texture::DepthFormat::F32,
                width,
                height,
            )
            .unwrap(),
        }
    }
}

#[derive(Copy, Clone)]
struct ColoredVertex {
    id: u32,
    position: [f32; 3],
    color: [f32; 3],
}
implement_vertex!(ColoredVertex, position, id);

#[derive(Copy, Clone)]
struct ScreenVertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}
implement_vertex!(ScreenVertex, position, tex_coords);

const SCREEN_QUAD: [ScreenVertex; 6] = [
    ScreenVertex {
        position: [-1.0, -1.0],
        tex_coords: [0.0, 0.0],
    },
    ScreenVertex {
        position: [1.0, -1.0],
        tex_coords: [1.0, 0.0],
    },
    ScreenVertex {
        position: [-1.0, 1.0],
        tex_coords: [0.0, 1.0],
    },
    ScreenVertex {
        position: [-1.0, 1.0],
        tex_coords: [0.0, 1.0],
    },
    ScreenVertex {
        position: [1.0, -1.0],
        tex_coords: [1.0, 0.0],
    },
    ScreenVertex {
        position: [1.0, 1.0],
        tex_coords: [1.0, 1.0],
    },
];

static INDICES: NoIndices = NoIndices(PrimitiveType::TrianglesList);

struct GliumQuads {
    vertex_buffer: VertexBuffer<ColoredVertex>,
}

impl GliumQuads {}

impl Graphics {
    pub fn draw_triangles(&mut self, triangles: &[Triangle]) -> usize {
        let id = match self.quad_ids.pop() {
            Some(id) => id,
            None => {
                let out = self
                    .quad_ids
                    .len()
                    .try_into()
                    .expect("Cannot allocate any more quads");
                self.quads.push(None);
                out
            }
        };

        let vertices = triangles
            .iter()
            .flat_map(|Triangle { id, corners, color }| {
                corners.iter().map(|corner| ColoredVertex {
                    id: *id,
                    position: *corner,
                    color: [color.r, color.g, color.b],
                })
            })
            .collect::<Vec<ColoredVertex>>();

        let vertex_buffer = VertexBuffer::new(&self.display, &vertices).unwrap();

        self.quads[id] = Some(GliumQuads { vertex_buffer });

        id
    }

    fn get_canvas(&mut self, dimensions: &(u32, u32)) -> Option<Canvas> {
        match self.canvas {
            Some(Canvas { width, height, .. }) => {
                if (width, height) == *dimensions {
                    return self.canvas.take();
                }
            }
            None => (),
        }
        Some(Canvas::new(&self.display, dimensions))
    }

    pub fn render(&mut self) {
        let mut frame = self.display.draw();

        self.canvas = self.get_canvas(&frame.get_dimensions());

        let canvas = self.canvas.as_ref().unwrap();

        canvas
            .texture
            .main_level()
            .first_layer()
            .into_image(None)
            .unwrap()
            .raw_clear_buffer([0.0f32, 0.0, 0.0, 0.0]);

        let matrix = self.matrices.composite;
        let uniforms = uniform! {
            matrix: [
                [matrix[(0, 0)], matrix[(1, 0)], matrix[(2, 0)], matrix[(3, 0)]],
                [matrix[(0, 1)], matrix[(1, 1)], matrix[(2, 1)], matrix[(3, 1)]],
                [matrix[(0, 2)], matrix[(1, 2)], matrix[(2, 2)], matrix[(3, 2)]],
                [matrix[(0, 3)], matrix[(1, 3)], matrix[(2, 3)], matrix[(3, 3)]],
            ],
            selection: 0u32
        };
        let mut frame_buffer = SimpleFrameBuffer::with_depth_buffer(
            &self.display,
            &canvas.texture,
            &canvas.depth_buffer,
        )
        .unwrap();
        frame_buffer.clear_depth(1.0);

        for quads in &self.quads {
            if let Some(quads) = quads {
                frame_buffer
                    .draw(
                        &quads.vertex_buffer,
                        &INDICES,
                        &self.programs.primitive,
                        &uniforms,
                        &self.draw_parameters,
                    )
                    .unwrap();
            }
        }

        let uniforms = uniform! {
            tex: &canvas.texture
        };

        frame
            .draw(
                &self.screen_vertices,
                &INDICES,
                &self.programs.screen,
                &uniforms,
                &Default::default(),
            )
            .unwrap();

        frame.finish().unwrap();
    }
}
