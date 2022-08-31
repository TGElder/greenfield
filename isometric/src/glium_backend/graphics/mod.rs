mod canvas;
mod matrices;
mod programs;
mod vertices;

use std::f32::consts::PI;

use crate::glium_backend::engine::Engine;
use crate::graphics::elements::Triangle;
use crate::graphics::GraphicsBackend;
use canvas::*;
use glium::glutin;
use glium::Surface;
use matrices::*;
use programs::*;
use vertices::*;

static INDICES: glium::index::NoIndices =
    glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

const SCREEN_QUAD: [ScreenVertex; 6] = [
    ScreenVertex {
        screen_position: [-1.0, -1.0],
        canvas_position: [0.0, 0.0],
    },
    ScreenVertex {
        screen_position: [1.0, -1.0],
        canvas_position: [1.0, 0.0],
    },
    ScreenVertex {
        screen_position: [-1.0, 1.0],
        canvas_position: [0.0, 1.0],
    },
    ScreenVertex {
        screen_position: [-1.0, 1.0],
        canvas_position: [0.0, 1.0],
    },
    ScreenVertex {
        screen_position: [1.0, -1.0],
        canvas_position: [1.0, 0.0],
    },
    ScreenVertex {
        screen_position: [1.0, 1.0],
        canvas_position: [1.0, 1.0],
    },
];

pub struct Graphics {
    display: glium::Display,
    matrices: Matrices,
    canvas: Option<Canvas>,
    screen_vertices: glium::VertexBuffer<ScreenVertex>,
    primitives: Vec<Option<Primitive>>,
    primitive_ids: Vec<usize>,
    programs: Programs,
    draw_parameters: glium::DrawParameters<'static>,
}

pub struct Parameters {
    pub name: String,
    pub width: f32,
    pub height: f32,
}

impl Graphics {
    pub fn with_engine(parameters: Parameters, engine: &Engine) -> Graphics {
        Self::with_event_loop(parameters, &engine.event_loop)
    }

    fn with_event_loop<T>(
        parameters: Parameters,
        event_loop: &glutin::event_loop::EventLoop<T>,
    ) -> Graphics {
        let window_builder = glutin::window::WindowBuilder::new()
            .with_inner_size(glutin::dpi::LogicalSize::new(
                parameters.width,
                parameters.height,
            ))
            .with_title(&parameters.name);
        let context_builder = glutin::ContextBuilder::new().with_depth_buffer(24);
        let display = glium::Display::new(window_builder, context_builder, event_loop).unwrap();
        Graphics {
            matrices: Matrices::new(PI / 4.0, 5.0 * PI / 8.0, 1.0 / 256.0),
            canvas: None,
            screen_vertices: glium::VertexBuffer::new(&display, &SCREEN_QUAD).unwrap(),
            primitives: vec![],
            primitive_ids: vec![],
            programs: Programs::new(&display),
            draw_parameters: glium::DrawParameters {
                depth: glium::Depth {
                    test: glium::DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },
                ..Default::default()
            },
            display,
        }
    }
}

struct Primitive {
    vertex_buffer: glium::VertexBuffer<ColoredVertex>,
}

impl GraphicsBackend for Graphics {
    fn add_primitive(&mut self, triangles: &[Triangle]) -> usize {
        let id = match self.primitive_ids.pop() {
            Some(id) => id,
            None => {
                let out = self.primitive_ids.len();
                self.primitives.push(None);
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
        self.primitives[id] = Some(Primitive {
            vertex_buffer: glium::VertexBuffer::new(&self.display, &vertices).unwrap(),
        });

        id
    }

    fn render(&mut self) {
        let mut frame = self.display.draw();

        self.canvas = self.canvas(&frame.get_dimensions());

        let canvas = self.canvas.as_ref().unwrap();
        let mut canvas = canvas.frame(&self.display);

        self.render_primitives_to_canvas(&mut canvas);
        self.render_canvas_to_frame(&mut frame);

        frame.finish().unwrap();
    }

    fn screenshot(&self, path: &str) {
        if let Some(canvas) = &self.canvas {
            canvas.save_texture(path);
        }
    }
}

impl Graphics {
    fn canvas(&mut self, dimensions: &(u32, u32)) -> Option<Canvas> {
        if let Some(Canvas { width, height, .. }) = self.canvas {
            if (width, height) == *dimensions {
                return self.canvas.take();
            }
        }
        Some(Canvas::new(&self.display, dimensions))
    }

    fn render_primitives_to_canvas<S>(&self, surface: &mut S)
    where
        S: glium::Surface,
    {
        let transform: [[f32; 4]; 4] = self.matrices.composite.into();
        let uniforms = glium::uniform! {
            transform: transform
        };

        for primitive in self.primitives.iter().flatten() {
            surface
                .draw(
                    &primitive.vertex_buffer,
                    &INDICES,
                    &self.programs.primitive,
                    &uniforms,
                    &self.draw_parameters,
                )
                .unwrap();
        }
    }

    fn render_canvas_to_frame<S>(&self, surface: &mut S)
    where
        S: glium::Surface,
    {
        let canvas = self.canvas.as_ref().unwrap();

        let uniforms = glium::uniform! {
            canvas: &canvas.texture
        };

        surface
            .draw(
                &self.screen_vertices,
                &INDICES,
                &self.programs.screen,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
    }
}
