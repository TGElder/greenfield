mod canvas;
mod programs;
#[cfg(test)]
mod tests;
mod vertices;

use std::error::Error;

use crate::graphics::elements::{self, Triangle};
use crate::graphics::errors::{
    DrawError, IndexError, InitializationError, RenderError, ScreenshotError,
};
use crate::graphics::projection::Projection;
use crate::graphics::Graphics;
use canvas::*;
use commons::geometry::{xy, xyz, XY, XYZ};
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

pub struct GliumGraphics {
    display: Display,
    projection: Box<dyn Projection>,
    canvas: Option<Canvas>,
    screen_vertices: glium::VertexBuffer<ScreenVertex>,
    textures: Vec<glium::Texture2d>,
    primitives: Vec<Option<Primitive>>,
    billboards: Vec<Option<Billboard>>,
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
            textures: vec![],
            primitives: vec![],
            billboards: vec![],
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

    fn render_billboards_to_canvas<S>(&self, surface: &mut S) -> Result<(), Box<dyn Error>>
    where
        S: glium::Surface,
    {
        let mut uniforms = None;
        let mut current_texture = None;

        for billboard in self.billboards.iter().flatten() {
            if current_texture != Some(billboard.texture) {
                current_texture = Some(billboard.texture);
                uniforms = Some(glium::uniform! {
                    transform: *self.projection.projection(),
                    scale: self.projection.scale(),
                    tex: self.textures.get(billboard.texture).ok_or(format!("Billboard refers to missing texture {}", billboard.texture))?
                });
            }
            if let Some(uniforms) = uniforms {
                surface.draw(
                    &billboard.vertex_buffer,
                    INDICES,
                    &self.programs.billboard,
                    &uniforms,
                    &self.draw_parameters,
                )?;
            }
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

    fn screen_to_gl(&self, XY { x, y }: &XY<u32>) -> XY<f32> {
        let (width, height) = self.display.canvas_dimensions();

        let x_pc = *x as f32 / width as f32;
        let y_pc = *y as f32 / height as f32;

        xy(x_pc * 2.0 - 1.0, -(y_pc * 2.0 - 1.0))
    }

    fn load_texture_unsafe(&mut self, path: &str) -> Result<usize, Box<dyn Error>> {
        let image = image::open(path)?.to_rgba8();
        let dimensions = image.dimensions();
        let image =
            glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), dimensions);

        let texture = glium::Texture2d::new(self.display.facade(), image).unwrap();
        self.textures.push(texture);
        Ok(self.textures.len() - 1)
    }

    fn add_triangles_unsafe(&mut self, triangles: &[Triangle]) -> Result<usize, Box<dyn Error>> {
        let vertices = triangles
            .iter()
            .flat_map(|Triangle { corners, color }| {
                corners.iter().map(|corner| ColoredVertex {
                    position: (*corner).into(),
                    color: [color.r, color.g, color.b],
                })
            })
            .collect::<Vec<ColoredVertex>>();

        self.primitives.push(Some(Primitive {
            vertex_buffer: glium::VertexBuffer::new(self.display.facade(), &vertices)?,
        }));

        Ok(self.primitives.len() - 1)
    }

    fn add_billboard_unsafe(
        &mut self,
        elements::Billboard {
            position,
            dimensions,
            texture,
        }: &elements::Billboard,
    ) -> Result<usize, Box<dyn Error>> {
        let vertices = [xy(0.0, 0.0), xy(1.0, 0.0), xy(1.0, 1.0), xy(0.0, 1.0)]
            .iter()
            .map(|o| BillboardVertex {
                position: (*position).into(),
                offset: [
                    dimensions.width * (o.x - 0.5),
                    dimensions.height * (o.y - 0.5),
                ],
                texture_coordinates: [o.x, o.y],
            })
            .collect::<Vec<BillboardVertex>>();

        let vertices = [
            vertices[0],
            vertices[1],
            vertices[3],
            vertices[1],
            vertices[2],
            vertices[3],
        ];

        self.billboards.push(Some(Billboard {
            texture: *texture,
            vertex_buffer: glium::VertexBuffer::new(self.display.facade(), &vertices)?,
        }));

        Ok(self.billboards.len() - 1)
    }

    fn render_unsafe(&mut self) -> Result<(), Box<dyn Error>> {
        let mut frame = self.display.frame();

        self.canvas = Some(self.canvas(&self.display.canvas_dimensions())?);
        let canvas = self.canvas.as_ref().unwrap();
        let mut canvas = canvas.frame(self.display.facade())?;

        self.render_primitives_to_canvas(&mut canvas)?;
        self.render_billboards_to_canvas(&mut canvas)?;
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

    fn world_xyz_at_unsafe(&self, screen_xy: &XY<u32>) -> Result<XYZ<f32>, Box<dyn Error>> {
        let Some(canvas) = &self.canvas else{return Err("Need the depth at the cursor position to get world coordinate, but there is no canvas to read the depth from.".into())};
        let gl_z = canvas.read_pixel(*screen_xy)?.a;
        let XY { x: gl_x, y: gl_y } = self.screen_to_gl(screen_xy);
        let gl_xyz = xyz(gl_x, gl_y, gl_z);
        Ok(self.projection.unproject(&gl_xyz))
    }
}

impl Graphics for GliumGraphics {
    fn load_texture(&mut self, path: &str) -> Result<usize, InitializationError> {
        Ok(self.load_texture_unsafe(path)?)
    }

    fn add_triangles(&mut self, triangles: &[Triangle]) -> Result<usize, DrawError> {
        Ok(self.add_triangles_unsafe(triangles)?)
    }

    fn add_billboard(&mut self, billboard: &elements::Billboard) -> Result<usize, DrawError> {
        Ok(self.add_billboard_unsafe(billboard)?)
    }

    fn render(&mut self) -> Result<(), RenderError> {
        Ok(self.render_unsafe()?)
    }

    fn screenshot(&self, path: &str) -> Result<(), ScreenshotError> {
        Ok(self.screenshot_unsafe(path)?)
    }

    fn look_at(&mut self, world_xyz: &XYZ<f32>, screen_xy: &XY<u32>) {
        let gl_xy = self.screen_to_gl(screen_xy);
        self.projection.look_at(world_xyz, &gl_xy)
    }

    fn world_xyz_at(&mut self, screen_xy: &XY<u32>) -> Result<XYZ<f32>, IndexError> {
        Ok(self.world_xyz_at_unsafe(screen_xy)?)
    }

    fn projection(&mut self) -> &mut Box<dyn Projection> {
        &mut self.projection
    }
}

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

struct Primitive {
    vertex_buffer: glium::VertexBuffer<ColoredVertex>,
}

struct Billboard {
    texture: usize,
    vertex_buffer: glium::VertexBuffer<BillboardVertex>,
}
