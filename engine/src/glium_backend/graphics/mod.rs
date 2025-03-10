mod canvas;
mod programs;
#[cfg(test)]
mod tests;
mod utils;
mod vertices;

use std::error::Error;
use std::mem;

use crate::glium_backend::graphics::utils::colored_vertices_from_triangles;
use crate::graphics::elements::{self, OverlayTriangles, TexturedPosition, Triangle};
use crate::graphics::errors::{
    DrawError, IndexError, InitializationError, RenderError, ScreenshotError,
};
use crate::graphics::projection::Projection;
use crate::graphics::{DrawMode, Graphics};
use canvas::*;
use commons::color::{Rgb, Rgba};
use commons::geometry::{xy, xyz, XY, XYZ};
use commons::grid::Grid;
use commons::origin_grid::OriginGrid;
use egui_glium::egui_winit::egui::{self, ViewportId};
use glium::glutin::surface::WindowSurface;
use glium::VertexBuffer;
use nalgebra::Matrix4;
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
    projection: Box<dyn Projection>,
    light_direction: [f32; 3],
    ambient_light: f32,
    canvas: Option<Canvas>,
    screen_vertices: glium::VertexBuffer<ScreenVertex>,
    textures: Vec<glium::Texture2d>,
    primitives: Vec<Option<Primitive>>,
    overlay_primitives: Vec<Option<OverlayPrimitive>>,
    instanced_primitives: Vec<Option<InstancedPrimitives>>,
    billboards: Vec<Option<Billboard>>,
    programs: Programs,
    draw_parameters: DrawParameters,
    gui: egui_glium::EguiGlium,
    display: glium::Display<WindowSurface>,
    window: winit::window::Window,
}

pub struct DrawParameters {
    solid: glium::DrawParameters<'static>,
    hologram: glium::DrawParameters<'static>,
}

pub struct Parameters {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub projection: Box<dyn Projection>,
    pub light_direction: XYZ<f32>,
    pub ambient_light: f32,
}

impl GliumGraphics {
    pub fn new<T>(
        parameters: Parameters,
        event_loop: &winit::event_loop::EventLoop<T>,
    ) -> Result<GliumGraphics, InitializationError> {
        Ok(Self::new_unsafe(parameters, event_loop)?)
    }

    pub(super) fn handle_window_event(
        &mut self,
        event: &winit::event::WindowEvent,
    ) -> egui_glium::EventResponse {
        let event_response = self.gui.on_event(&self.window, event);

        if event_response.repaint {
            self.window.request_redraw();
        }

        event_response
    }

    fn new_unsafe<T>(
        parameters: Parameters,
        event_loop: &winit::event_loop::EventLoop<T>,
    ) -> Result<GliumGraphics, Box<dyn Error>> {
        let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
            .set_window_builder(winit::window::WindowBuilder::new().with_resizable(true))
            .with_inner_size(parameters.width, parameters.height)
            .with_title(&parameters.name)
            .build(event_loop);
        let default_draw_parameters = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            ..Default::default()
        };

        let mut out = GliumGraphics {
            projection: parameters.projection,
            light_direction: parameters.light_direction.into(),
            ambient_light: parameters.ambient_light,
            canvas: None,
            screen_vertices: glium::VertexBuffer::new(&display, &SCREEN_QUAD)?,
            textures: vec![],
            primitives: vec![],
            overlay_primitives: vec![],
            billboards: vec![],
            instanced_primitives: vec![],
            programs: Programs::new(&display)?,
            draw_parameters: DrawParameters {
                solid: glium::DrawParameters {
                    color_mask: (true, true, true, true),
                    ..default_draw_parameters.clone()
                },
                hologram: glium::DrawParameters {
                    color_mask: (true, true, true, false),
                    ..default_draw_parameters
                },
            },
            gui: egui_glium::EguiGlium::new(ViewportId::ROOT, &display, &window, event_loop),
            display,
            window,
        };

        out.draw_gui(&mut |_| {}); // we get an error on gui.paint if we don't do this

        Ok(out)
    }

    fn canvas(&mut self, dimensions: &(u32, u32)) -> Result<Canvas, Box<dyn Error>> {
        if let Some(Canvas { width, height, .. }) = self.canvas {
            if (width, height) == *dimensions {
                return Ok(self.canvas.take().unwrap());
            }
        }
        Canvas::new(&self.display, &self.display.get_framebuffer_dimensions())
    }

    fn draw_parameters(&self, draw_mode: DrawMode) -> Option<&glium::DrawParameters> {
        match draw_mode {
            DrawMode::Solid => Some(&self.draw_parameters.solid),
            DrawMode::Hologram => Some(&self.draw_parameters.hologram),
            _ => None,
        }
    }

    fn render_primitives_to_canvas<S>(
        &self,
        draw_mode: DrawMode,
        surface: &mut S,
        primitives: &[Option<Primitive>],
    ) -> Result<(), Box<dyn Error>>
    where
        S: glium::Surface,
    {
        let draw_parameters = self
            .draw_parameters(draw_mode)
            .ok_or(format!("No draw parameters for draw mode {:?}", draw_mode))?;

        let uniforms = glium::uniform! {
            transform: self.projection.projection(),
            light_direction: self.light_direction,
            ambient_light: self.ambient_light
        };

        for primitive in primitives
            .iter()
            .flatten()
            .filter(|primitive| primitive.draw_mode == draw_mode)
        {
            surface.draw(
                &primitive.vertex_buffer,
                INDICES,
                &self.programs.primitive,
                &uniforms,
                draw_parameters,
            )?;
        }

        Ok(())
    }

    fn render_overlay_primitives_to_canvas<S>(
        &self,
        draw_mode: DrawMode,
        surface: &mut S,
    ) -> Result<(), Box<dyn Error>>
    where
        S: glium::Surface,
    {
        let draw_parameters = self
            .draw_parameters(draw_mode)
            .ok_or(format!("No draw parameters for draw mode {:?}", draw_mode))?;

        let mut uniforms = None;
        let mut current_base_texture = None;
        let mut current_overlay_texture = None;

        let sampler_behavior = glium::uniforms::SamplerBehavior {
            minify_filter: glium::uniforms::MinifySamplerFilter::Nearest,
            magnify_filter: glium::uniforms::MagnifySamplerFilter::Nearest,
            ..Default::default()
        };

        for primitive in self
            .overlay_primitives
            .iter()
            .flatten()
            .filter(|primitive| primitive.draw_mode == draw_mode)
        {
            if current_base_texture != Some(primitive.base_texture)
                || current_overlay_texture != Some(primitive.overlay_texture)
            {
                current_base_texture = Some(primitive.base_texture);
                current_overlay_texture = Some(primitive.overlay_texture);
                let base = self.textures.get(primitive.base_texture).ok_or(format!(
                    "Overlay primitive refers to missing base texture {}",
                    primitive.base_texture
                ))?;
                let base = glium::uniforms::Sampler(base, sampler_behavior);
                let overlay = self.textures.get(primitive.overlay_texture).ok_or(format!(
                    "Overlay primitive refers to missing overlay texture {}",
                    primitive.overlay_texture
                ))?;
                let overlay = glium::uniforms::Sampler(overlay, sampler_behavior);
                uniforms = Some(glium::uniform! {
                    transform: self.projection.projection(),
                    light_direction: self.light_direction,
                    ambient_light: self.ambient_light,
                    base: base,
                    overlay: overlay,
                });
            }
            if let Some(uniforms) = uniforms {
                surface.draw(
                    &primitive.vertex_buffer,
                    INDICES,
                    &self.programs.overlay_primitive,
                    &uniforms,
                    draw_parameters,
                )?;
            }
        }
        Ok(())
    }

    fn render_billboards_to_canvas<S>(
        &self,
        draw_mode: DrawMode,
        surface: &mut S,
    ) -> Result<(), Box<dyn Error>>
    where
        S: glium::Surface,
    {
        let draw_parameters = self
            .draw_parameters(draw_mode)
            .ok_or(format!("No draw parameters for draw mode {:?}", draw_mode))?;

        let mut uniforms = None;
        let mut current_texture = None;

        let sampler_behavior = glium::uniforms::SamplerBehavior {
            minify_filter: glium::uniforms::MinifySamplerFilter::Nearest,
            magnify_filter: glium::uniforms::MagnifySamplerFilter::Nearest,
            ..Default::default()
        };

        for billboard in self
            .billboards
            .iter()
            .flatten()
            .filter(|billboard| billboard.draw_mode == draw_mode)
        {
            if current_texture != Some(billboard.texture) {
                current_texture = Some(billboard.texture);
                let texture = self.textures.get(billboard.texture).ok_or(format!(
                    "Billboard refers to missing texture {}",
                    billboard.texture
                ))?;
                let sampler = glium::uniforms::Sampler(texture, sampler_behavior);
                uniforms = Some(glium::uniform! {
                    transform: self.projection.projection(),
                    scale: self.projection.scale(),
                    tex: sampler
                });
            }
            if let Some(uniforms) = uniforms {
                surface.draw(
                    &billboard.vertex_buffer,
                    INDICES,
                    &self.programs.billboard,
                    &uniforms,
                    draw_parameters,
                )?;
            }
        }

        Ok(())
    }

    fn render_instanced_primitives_to_canvas<S>(
        &self,
        draw_mode: DrawMode,
        surface: &mut S,
    ) -> Result<(), Box<dyn Error>>
    where
        S: glium::Surface,
    {
        let draw_parameters = self
            .draw_parameters(draw_mode)
            .ok_or(format!("No draw parameters for draw mode {:?}", draw_mode))?;

        let uniforms = glium::uniform! {
            transform: self.projection.projection(),
            light_direction: self.light_direction,
            ambient_light: self.ambient_light,
        };

        for InstancedPrimitives {
            primitive,
            vertex_buffer,
            ..
        } in self
            .instanced_primitives
            .iter()
            .flatten()
            .filter(|primitives| primitives.primitive.draw_mode == draw_mode)
        {
            let Some(vertex_buffer) = vertex_buffer else {
                continue;
            };
            surface.draw(
                (
                    &primitive.vertex_buffer,
                    vertex_buffer
                        .per_instance()
                        .map_err(|_| String::from("Instancing is not supported"))?,
                ),
                INDICES,
                &self.programs.instanced_primitive,
                &uniforms,
                draw_parameters,
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

    fn screen_to_gl(&self, XY { x, y }: &XY<u32>) -> XY<f32> {
        let (width, height) = self.display.get_framebuffer_dimensions();

        let x_pc = *x as f32 / width as f32;
        let y_pc = *y as f32 / height as f32;

        xy(x_pc * 2.0 - 1.0, -(y_pc * 2.0 - 1.0))
    }

    fn load_texture_unsafe(&mut self, image: &Grid<Rgba<u8>>) -> Result<usize, Box<dyn Error>> {
        let raw = image
            .iter()
            .map(|xy| image[xy])
            .flat_map(|Rgba { r, g, b, a }| [r, g, b, a])
            .collect::<Vec<_>>();
        let dimensions = (image.width(), image.height());
        let image = glium::texture::RawImage2d::from_raw_rgba(raw, dimensions);

        let texture = glium::Texture2d::new(&self.display, image).unwrap();
        self.textures.push(texture);
        Ok(self.textures.len() - 1)
    }

    fn load_texture_from_file_unsafe(&mut self, path: &str) -> Result<usize, Box<dyn Error>> {
        let image = image::open(path)?.to_rgba8();
        let dimensions = image.dimensions();
        let image =
            glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), dimensions);

        let texture = glium::Texture2d::new(&self.display, image).unwrap();
        self.textures.push(texture);
        Ok(self.textures.len() - 1)
    }

    fn modify_texture_unsafe(
        &mut self,
        index: &usize,
        image: &OriginGrid<Rgba<u8>>,
    ) -> Result<(), Box<dyn Error>> {
        if *index >= self.textures.len() {
            return Err(format!(
                "Trying to modify texture #{} but there are only {} textures",
                index,
                self.textures.len()
            )
            .into());
        }
        let texture = &mut self.textures[*index];
        let rect = glium::Rect {
            left: image.origin().x,
            bottom: image.origin().y,
            width: image.width(),
            height: image.height(),
        };
        let mut data = Vec::with_capacity(image.height() as usize);
        for y in 0..image.height() {
            data.push(Vec::with_capacity(image.width() as usize));
            for x in 0..image.width() {
                let Rgba { r, g, b, a } = image[xy(x + image.origin().x, y + image.origin().y)];
                data[y as usize].push((r, g, b, a));
            }
        }
        texture.write(rect, data);
        Ok(())
    }

    fn create_triangles_unsafe(&mut self) -> Result<usize, Box<dyn Error>> {
        if self.primitives.len() == isize::MAX as usize {
            return Err("No space for more primitives".into());
        }
        self.primitives.push(None);
        Ok(self.primitives.len() - 1)
    }

    fn create_dynamic_triangles_unsafe(
        &mut self,
        triangles: &usize,
    ) -> Result<usize, Box<dyn Error>> {
        if self.primitives.len() == isize::MAX as usize {
            return Err("No space for more primitives".into());
        }
        let primitive = Primitive {
            draw_mode: DrawMode::Invisible,
            vertex_buffer: glium::VertexBuffer::dynamic(
                &self.display,
                &vec![ColoredVertex::default(); triangles * 3],
            )?,
        };
        self.primitives.push(Some(primitive));
        Ok(self.primitives.len() - 1)
    }

    fn create_overlay_triangles_unsafe(&mut self) -> Result<usize, Box<dyn Error>> {
        if self.overlay_primitives.len() == isize::MAX as usize {
            return Err("No space for more overlay_primitives".into());
        }
        self.overlay_primitives.push(None);
        Ok(self.overlay_primitives.len() - 1)
    }

    fn create_instanced_triangles_unsafe(
        &mut self,
        draw_mode: DrawMode,
        triangles: &[Triangle<Rgb<f32>>],
    ) -> Result<usize, Box<dyn Error>> {
        if self.instanced_primitives.len() == isize::MAX as usize {
            return Err("No space for more instanced_primitives".into());
        }

        let primitive_vertices = colored_vertices_from_triangles(triangles);
        let instanced_primitives = InstancedPrimitives {
            primitive: Primitive {
                draw_mode,
                vertex_buffer: glium::VertexBuffer::new(&self.display, &primitive_vertices)?,
            },
            vertex_buffer: None,
        };
        self.instanced_primitives.push(Some(instanced_primitives));

        Ok(self.instanced_primitives.len() - 1)
    }

    fn create_billboards_unsafe(&mut self) -> Result<usize, Box<dyn Error>> {
        if self.billboards.len() == isize::MAX as usize {
            return Err("No space for more billboards".into());
        }
        self.billboards.push(None);
        Ok(self.billboards.len() - 1)
    }

    fn add_triangles_unsafe(
        &mut self,
        index: &usize,
        draw_mode: DrawMode,
        triangles: &[Triangle<Rgb<f32>>],
    ) -> Result<(), Box<dyn Error>> {
        if *index >= self.primitives.len() {
            return Err(format!(
                "Trying to draw primitive #{} but there are only {} primitives",
                index,
                self.primitives.len()
            )
            .into());
        }

        let vertices = colored_vertices_from_triangles(triangles);
        let primitive = Primitive {
            draw_mode,
            vertex_buffer: VertexBuffer::new(&self.display, &vertices)?,
        };

        self.primitives[*index] = Some(primitive);

        Ok(())
    }

    fn update_dynamic_triangles_unsafe(
        &mut self,
        index: &usize,
        draw_mode: DrawMode,
        triangles: &[Triangle<Rgb<f32>>],
    ) -> Result<(), Box<dyn Error>> {
        if *index >= self.primitives.len() {
            return Err(format!(
                "Trying to draw dynamic primitive #{} but there are only {} dynamic primitives",
                index,
                self.primitives.len()
            )
            .into());
        }

        let primitive = &mut self.primitives[*index].as_mut().unwrap();

        match draw_mode {
            DrawMode::Invisible => {}
            _ => {
                let vertices = colored_vertices_from_triangles(triangles);
                primitive.vertex_buffer.write(&vertices);
            }
        }

        primitive.draw_mode = draw_mode;

        Ok(())
    }

    fn add_overlay_triangles_unsafe(
        &mut self,
        index: &usize,
        draw_mode: DrawMode,
        overlay: &OverlayTriangles,
    ) -> Result<(), Box<dyn Error>> {
        if *index >= self.overlay_primitives.len() {
            return Err(format!(
                "Trying to draw overlay primitive #{} but there are only {} overlay primitives",
                index,
                self.overlay_primitives.len()
            )
            .into());
        }

        let vertices = overlay
            .triangles
            .iter()
            .flat_map(|corners| {
                corners.iter().map(
                    |TexturedPosition {
                         position,
                         normal,
                         texture_coordinates,
                     }| TexturedVertex {
                        position: (*position).into(),
                        normal: (*normal).into(),
                        texture_coordinates: (*texture_coordinates).into(),
                    },
                )
            })
            .collect::<Vec<TexturedVertex>>();

        self.overlay_primitives[*index] = Some(OverlayPrimitive {
            draw_mode,
            base_texture: overlay.base_texture,
            overlay_texture: overlay.overlay_texture,
            vertex_buffer: glium::VertexBuffer::new(&self.display, &vertices)?,
        });

        Ok(())
    }

    fn update_instanced_triangles_unsafe(
        &mut self,
        index: &usize,
        world_matrices: &[Option<Matrix4<f32>>],
    ) -> Result<(), Box<dyn Error>> {
        if *index >= self.instanced_primitives.len() {
            return Err(format!(
                "Trying to update instanced triangles #{} but there are only {} instanced triangles",
                index,
                self.instanced_primitives.len()
            )
            .into());
        }

        let Some(instanced_primitives) = &mut self.instanced_primitives[*index] else {
            return Ok(());
        };

        let instances = world_matrices
            .iter()
            .flat_map(|matrix| {
                if let Some(matrix) = matrix {
                    Some(InstanceVertex {
                        world_matrix: (*matrix).into(),
                        world_normal_matrix: (matrix.try_inverse()?.transpose()).into(),
                    })
                } else {
                    Some(InstanceVertex::default())
                }
            })
            .collect::<Vec<_>>();

        match (instances.as_slice(), &instanced_primitives.vertex_buffer) {
            ([], _) => instanced_primitives.vertex_buffer = None,
            // reusing buffer
            (instances, Some(vertex_buffer))
                if (mem::size_of_val(instances) == vertex_buffer.get_size()) =>
            {
                vertex_buffer.write(instances)
            }
            // reallocating (expensive - keep number of instances the same where possible)
            _ => {
                instanced_primitives.vertex_buffer =
                    Some(glium::VertexBuffer::dynamic(&self.display, &instances)?)
            }
        }

        Ok(())
    }

    fn add_billboard_unsafe(
        &mut self,
        index: &usize,
        draw_mode: DrawMode,
        elements::Billboard {
            position,
            dimensions,
            texture,
        }: &elements::Billboard,
    ) -> Result<(), Box<dyn Error>> {
        if *index >= self.billboards.len() {
            return Err(format!(
                "Trying to draw billboard #{} but there are only {} billboards",
                index,
                self.billboards.len()
            )
            .into());
        }

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

        self.billboards[*index] = Some(Billboard {
            draw_mode,
            texture: *texture,
            vertex_buffer: glium::VertexBuffer::new(&self.display, &vertices)?,
        });

        Ok(())
    }

    fn render_unsafe(&mut self) -> Result<(), Box<dyn Error>> {
        let mut frame = self.display.draw();
        self.canvas = Some(self.canvas(&self.display.get_framebuffer_dimensions())?);
        let canvas = self.canvas.as_ref().unwrap();
        let mut canvas = canvas.frame(&self.display)?;

        for draw_mode in [DrawMode::Solid, DrawMode::Hologram] {
            self.render_primitives_to_canvas(draw_mode, &mut canvas, &self.primitives)?;
            self.render_overlay_primitives_to_canvas(draw_mode, &mut canvas)?;
            self.render_instanced_primitives_to_canvas(draw_mode, &mut canvas)?;
            self.render_billboards_to_canvas(draw_mode, &mut canvas)?;
        }

        self.render_canvas_to_frame(&mut frame)?;

        self.gui.paint(&self.display, &mut frame);

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
        let Some(canvas) = &self.canvas else {
            return Err("Need the depth at the cursor position to get world coordinate, but there is no canvas to read the depth from.".into());
        };
        let gl_z = canvas.read_pixel(*screen_xy)?.a;
        let XY { x: gl_x, y: gl_y } = self.screen_to_gl(screen_xy);
        let gl_xyz = xyz(gl_x, gl_y, gl_z);
        Ok(self.projection.unproject(&gl_xyz))
    }
}

impl Graphics for GliumGraphics {
    fn load_texture(&mut self, image: &Grid<Rgba<u8>>) -> Result<usize, InitializationError> {
        Ok(self.load_texture_unsafe(image)?)
    }

    fn load_texture_from_file(&mut self, path: &str) -> Result<usize, InitializationError> {
        Ok(self.load_texture_from_file_unsafe(path)?)
    }

    fn modify_texture(
        &mut self,
        id: &usize,
        image: &OriginGrid<Rgba<u8>>,
    ) -> Result<(), DrawError> {
        Ok(self.modify_texture_unsafe(id, image)?)
    }

    fn create_triangles(&mut self) -> Result<usize, IndexError> {
        Ok(self.create_triangles_unsafe()?)
    }

    fn create_dynamic_triangles(&mut self, triangles: &usize) -> Result<usize, IndexError> {
        Ok(self.create_dynamic_triangles_unsafe(triangles)?)
    }

    fn create_overlay_triangles(&mut self) -> Result<usize, IndexError> {
        Ok(self.create_overlay_triangles_unsafe()?)
    }

    fn create_instanced_triangles(
        &mut self,
        draw_mode: DrawMode,
        triangles: &[Triangle<Rgb<f32>>],
    ) -> Result<usize, IndexError> {
        Ok(self.create_instanced_triangles_unsafe(draw_mode, triangles)?)
    }

    fn create_billboards(&mut self) -> Result<usize, IndexError> {
        Ok(self.create_billboards_unsafe()?)
    }

    fn draw_triangles(
        &mut self,
        index: &usize,
        draw_mode: DrawMode,
        triangles: &[Triangle<Rgb<f32>>],
    ) -> Result<(), DrawError> {
        Ok(self.add_triangles_unsafe(index, draw_mode, triangles)?)
    }

    fn update_dynamic_triangles(
        &mut self,
        index: &usize,
        draw_mode: DrawMode,
        triangles: &[Triangle<Rgb<f32>>],
    ) -> Result<(), DrawError> {
        Ok(self.update_dynamic_triangles_unsafe(index, draw_mode, triangles)?)
    }

    fn draw_overlay_triangles(
        &mut self,
        index: &usize,
        draw_mode: DrawMode,
        overlay_triangles: &OverlayTriangles,
    ) -> Result<(), DrawError> {
        Ok(self.add_overlay_triangles_unsafe(index, draw_mode, overlay_triangles)?)
    }

    fn update_instanced_triangles(
        &mut self,
        index: &usize,
        world_matrices: &[Option<Matrix4<f32>>],
    ) -> Result<(), DrawError> {
        Ok(self.update_instanced_triangles_unsafe(index, world_matrices)?)
    }

    fn draw_billboard(
        &mut self,
        index: &usize,
        draw_mode: DrawMode,
        billboard: &elements::Billboard,
    ) -> Result<(), DrawError> {
        Ok(self.add_billboard_unsafe(index, draw_mode, billboard)?)
    }

    fn draw_gui(&mut self, run_ui: &mut dyn FnMut(&egui::Context)) {
        self.gui.run(&self.window, |egui_ctx| run_ui(egui_ctx));
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

    fn clear(&mut self) {
        self.textures.clear();
        self.primitives.clear();
        self.overlay_primitives.clear();
        self.billboards.clear();
        self.instanced_primitives.clear();
    }
}

struct Primitive {
    draw_mode: DrawMode,
    vertex_buffer: glium::VertexBuffer<ColoredVertex>,
}

struct OverlayPrimitive {
    draw_mode: DrawMode,
    base_texture: usize,
    overlay_texture: usize,
    vertex_buffer: glium::VertexBuffer<TexturedVertex>,
}

struct InstancedPrimitives {
    primitive: Primitive,
    vertex_buffer: Option<glium::VertexBuffer<InstanceVertex>>,
}

struct Billboard {
    draw_mode: DrawMode,
    texture: usize,
    vertex_buffer: glium::VertexBuffer<BillboardVertex>,
}
