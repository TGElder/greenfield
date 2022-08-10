mod canvas;
mod matrices;
mod programs;
mod vertices;

use std::f32::consts::PI;

use crate::graphics::elements::Triangle;
use crate::graphics::GraphicsBackend;
use canvas::*;
use matrices::*;
use programs::*;
use vertices::*;

static INDICES: glium::index::NoIndices =
    glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

pub struct Graphics {
    display: glium::Display,
    matrices: Matrices,
    canvas: Option<Canvas>,
    screen_vertices: glium::VertexBuffer<ScreenVertex>,
    quads: Vec<Option<GliumTriangles>>,
    quad_ids: Vec<usize>,
    programs: Programs,
    draw_parameters: glium::DrawParameters<'static>,
}

impl Graphics {
    pub fn new(display: glium::Display) -> Graphics {
        Graphics {
            matrices: Matrices::new(PI / 4.0, 5.0 * PI / 8.0),
            canvas: None,
            screen_vertices: glium::VertexBuffer::new(&display, &SCREEN_QUAD).unwrap(),
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

struct GliumTriangles {
    vertex_buffer: glium::VertexBuffer<ColoredVertex>,
}

impl GliumTriangles {}

impl GraphicsBackend for Graphics {
    fn draw_triangles(&mut self, triangles: &[Triangle]) -> usize {
        let id = match self.quad_ids.pop() {
            Some(id) => id,
            None => {
                let out = self.quad_ids.len();
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

        let vertex_buffer = glium::VertexBuffer::new(&self.display, &vertices).unwrap();

        self.quads[id] = Some(GliumTriangles { vertex_buffer });

        id
    }

    fn render(&mut self) {
        let mut frame = self.display.draw();

        self.canvas = self.get_canvas(&glium::Surface::get_dimensions(&frame));

        let canvas = self.canvas.as_ref().unwrap();

        canvas
            .texture
            .main_level()
            .first_layer()
            .into_image(None)
            .unwrap()
            .raw_clear_buffer([0.0f32, 0.0, 0.0, 0.0]);

        let transform = self.matrices.composite;
        let uniforms = glium::uniform! {
            matrix: [
                [transform[(0, 0)], transform[(1, 0)], transform[(2, 0)], transform[(3, 0)]],
                [transform[(0, 1)], transform[(1, 1)], transform[(2, 1)], transform[(3, 1)]],
                [transform[(0, 2)], transform[(1, 2)], transform[(2, 2)], transform[(3, 2)]],
                [transform[(0, 3)], transform[(1, 3)], transform[(2, 3)], transform[(3, 3)]],
            ],
            selection: 0u32
        };
        let mut frame_buffer = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(
            &self.display,
            &canvas.texture,
            &canvas.depth_buffer,
        )
        .unwrap();
        glium::Surface::clear_depth(&mut frame_buffer, 1.0);

        for quads in self.quads.iter().flatten() {
            glium::Surface::draw(
                &mut frame_buffer,
                &quads.vertex_buffer,
                &INDICES,
                &self.programs.primitive,
                &uniforms,
                &self.draw_parameters,
            )
            .unwrap();
        }

        let uniforms = glium::uniform! {
            tex: &canvas.texture
        };

        glium::Surface::draw(
            &mut frame,
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

impl Graphics {
    fn get_canvas(&mut self, dimensions: &(u32, u32)) -> Option<Canvas> {
        if let Some(Canvas { width, height, .. }) = self.canvas {
            if (width, height) == *dimensions {
                return self.canvas.take();
            }
        }
        Some(Canvas::new(&self.display, dimensions))
    }
}
