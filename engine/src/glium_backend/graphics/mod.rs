mod canvas;
mod programs;
#[cfg(test)]
mod tests;
mod vertices;

use std::error::Error;

use crate::graphics::elements::Triangle;
use crate::graphics::errors::{
    DrawError, IndexError, InitializationError, RenderError, ScreenshotError,
};
use crate::graphics::projection::{Projection, Rectangle};
use crate::graphics::Graphics;
use canvas::*;
use glium::glutin;
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

enum Display {
    Headful(glium::Display),
    Headless {
        renderer: glium::HeadlessRenderer,
        dimensions: (u32, u32),
    },
}

impl Display {
    fn facade(&self) -> &dyn glium::backend::Facade {
        match self {
            Display::Headful(display) => display,
            Display::Headless { renderer, .. } => renderer,
        }
    }

    fn frame(&self) -> glium::Frame {
        match self {
            Display::Headful(display) => display.draw(),
            Display::Headless { renderer, .. } => renderer.draw(),
        }
    }

    fn canvas_dimensions(&self) -> (u32, u32) {
        match self {
            Display::Headful(display) => display.get_framebuffer_dimensions(),
            Display::Headless { dimensions, .. } => *dimensions,
        }
    }
}

pub struct GliumGraphics {
    display: Display,
    projection: Box<dyn Projection>,
    canvas: Option<Canvas>,
    screen_vertices: glium::VertexBuffer<ScreenVertex>,
    primitives: Vec<Option<Primitive>>,
    primitive_indices: Vec<usize>,
    programs: Programs,
    draw_parameters: glium::DrawParameters<'static>,
}

pub struct Parameters {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub projection: Box<dyn Projection>,
}

impl GliumGraphics {
    pub fn headful<T>(
        parameters: Parameters,
        event_loop: &glutin::event_loop::EventLoop<T>,
    ) -> Result<GliumGraphics, InitializationError> {
        Ok(Self::headful_unsafe(parameters, event_loop)?)
    }

    fn headful_unsafe<T>(
        parameters: Parameters,
        event_loop: &glutin::event_loop::EventLoop<T>,
    ) -> Result<GliumGraphics, Box<dyn Error>> {
        let window_builder = glutin::window::WindowBuilder::new()
            .with_inner_size(glutin::dpi::PhysicalSize::new(
                parameters.width,
                parameters.height,
            ))
            .with_title(&parameters.name);
        let context_builder = glutin::ContextBuilder::new().with_depth_buffer(24);
        let display = glium::Display::new(window_builder, context_builder, event_loop)?;
        Self::new(parameters, Display::Headful(display))
    }

    pub fn headless(parameters: Parameters) -> Result<GliumGraphics, InitializationError> {
        Ok(Self::headless_unsafe(parameters)?)
    }

    fn headless_unsafe(parameters: Parameters) -> Result<GliumGraphics, Box<dyn Error>> {
        let ctx = glutin::platform::unix::HeadlessContextExt::build_osmesa(
            glutin::ContextBuilder::new(),
            glutin::dpi::PhysicalSize::new(parameters.width, parameters.height),
        )?;
        let renderer = glium::HeadlessRenderer::new(ctx)?;
        let display = Display::Headless {
            renderer,
            dimensions: (parameters.width, parameters.height),
        };
        Self::new(parameters, display)
    }

    fn new(parameters: Parameters, display: Display) -> Result<GliumGraphics, Box<dyn Error>> {
        Ok(GliumGraphics {
            projection: parameters.projection,
            canvas: None,
            screen_vertices: glium::VertexBuffer::new(display.facade(), &SCREEN_QUAD)?,
            primitives: vec![],
            primitive_indices: vec![],
            programs: Programs::new(display.facade())?,
            draw_parameters: glium::DrawParameters {
                depth: glium::Depth {
                    test: glium::DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },
                backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
                ..Default::default()
            },
            display,
        })
    }

    fn canvas(&mut self, dimensions: &(u32, u32)) -> Result<Canvas, Box<dyn Error>> {
        if let Some(Canvas { width, height, .. }) = self.canvas {
            if (width, height) == *dimensions {
                return Ok(self.canvas.take().unwrap());
            }
        }
        Canvas::new(self.display.facade(), &self.display.canvas_dimensions())
    }

    fn render_primitives_to_canvas<S>(&self, surface: &mut S) -> Result<(), Box<dyn Error>>
    where
        S: glium::Surface,
    {
        let uniforms = glium::uniform! {
            transform: *self.projection.projection()
        };

        for primitive in self.primitives.iter().flatten() {
            surface.draw(
                &primitive.vertex_buffer,
                INDICES,
                &self.programs.primitive,
                &uniforms,
                &self.draw_parameters,
            )?;
        }
        Ok(())
    }

    fn render_canvas_to_frame<S>(&self, surface: &mut S) -> Result<(), Box<dyn Error>>
    where
        S: glium::Surface,
    {
        let canvas = self.canvas.as_ref().unwrap();

        let uniforms = glium::uniform! {
            canvas: &canvas.texture
        };

        surface.draw(
            &self.screen_vertices,
            INDICES,
            &self.programs.screen,
            &uniforms,
            &Default::default(),
        )?;

        Ok(())
    }

    fn screen_to_gl(&self, (x, y): &(u32, u32)) -> [f32; 2] {
        let (width, height) = self.display.canvas_dimensions();

        let x_pc = *x as f32 / width as f32;
        let y_pc = *y as f32 / height as f32;

        [x_pc * 2.0 - 1.0, -(y_pc * 2.0 - 1.0)]
    }

    fn add_triangles_unsafe(&mut self, triangles: &[Triangle]) -> Result<usize, Box<dyn Error>> {
        let index = match self.primitive_indices.pop() {
            Some(index) => index,
            None => {
                let out = self.primitive_indices.len();
                self.primitives.push(None);
                out
            }
        };

        let vertices = triangles
            .iter()
            .flat_map(|Triangle { corners, color }| {
                corners.iter().map(|corner| ColoredVertex {
                    position: *corner,
                    color: [color.r, color.g, color.b],
                })
            })
            .collect::<Vec<ColoredVertex>>();

        self.primitives[index] = Some(Primitive {
            vertex_buffer: glium::VertexBuffer::new(self.display.facade(), &vertices)?,
        });

        Ok(index)
    }

    fn render_unsafe(&mut self) -> Result<(), Box<dyn Error>> {
        let mut frame = self.display.frame();

        self.canvas = Some(self.canvas(&self.display.canvas_dimensions())?);
        let canvas = self.canvas.as_ref().unwrap();
        let mut canvas = canvas.frame(self.display.facade())?;

        self.render_primitives_to_canvas(&mut canvas)?;
        self.render_canvas_to_frame(&mut frame)?;

        frame.finish()?;

        Ok(())
    }

    fn screenshot_unsafe(&self, path: &str) -> Result<(), Box<dyn Error>> {
        if let Some(canvas) = &self.canvas {
            canvas.save_texture(path)?;
        }
        Ok(())
    }

    fn world_xyz_at_unsafe(&self, screen_xy: &(u32, u32)) -> Result<[f32; 3], Box<dyn Error>> {
        let Some(canvas) = &self.canvas else{return Err("Need the depth at the cursor position to get world coordinate, but there is no canvas to read the depth from.".into())};
        let gl_z = canvas.read_pixel(*screen_xy)?.a;
        let [gl_x, gl_y] = self.screen_to_gl(screen_xy);
        let gl_xyz = [gl_x, gl_y, gl_z];
        Ok(self.projection.unproject(&gl_xyz))
    }
}

struct Primitive {
    vertex_buffer: glium::VertexBuffer<ColoredVertex>,
}

impl Graphics for GliumGraphics {
    fn add_triangles(&mut self, triangles: &[Triangle]) -> Result<usize, DrawError> {
        Ok(self.add_triangles_unsafe(triangles)?)
    }

    fn render(&mut self) -> Result<(), RenderError> {
        Ok(self.render_unsafe()?)
    }

    fn screenshot(&self, path: &str) -> Result<(), ScreenshotError> {
        Ok(self.screenshot_unsafe(path)?)
    }

    fn look_at(&mut self, world_xyz: &[f32; 3], screen_xy: &(u32, u32)) {
        let gl_xy = self.screen_to_gl(screen_xy);
        self.projection.look_at(world_xyz, &gl_xy)
    }

    fn yaw(&mut self, yaw: f32) {
        self.projection.yaw(yaw);
    }

    fn zoom(&mut self, zoom: f32) {
        self.projection.zoom(zoom);
    }

    fn set_viewport_size(&mut self, viewport_size: Rectangle) {
        self.projection.set_viewport_size(viewport_size);
    }

    fn world_xyz_at(&mut self, screen_xy: &(u32, u32)) -> Result<[f32; 3], IndexError> {
        Ok(self.world_xyz_at_unsafe(screen_xy)?)
    }
}
