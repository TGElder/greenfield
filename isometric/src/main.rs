extern crate glium;

use std::f32::consts::PI;

use commons::grid::Grid;
use commons::noise::simplex_noise;
use glium::{glutin, implement_vertex};
use glium::{uniform, Surface};
use terrain_gen::with_valleys::{heightmap_from_rises_with_valleys, ValleyParameters};

#[derive(Copy, Clone)]
struct Vertex2 {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

#[derive(Copy, Clone)]
struct Vertex3 {
    position: [f32; 3],
    id: u32,
}

implement_vertex!(Vertex2, position, tex_coords);
implement_vertex!(Vertex3, position, id);

fn main() {
    // 1. The **winit::EventsLoop** for handling events.
    let event_loop = glium::glutin::event_loop::EventLoop::new();
    // 2. Parameters for building the Window.
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(1024.0, 768.0))
        .with_title("Hello world");
    // 3. Parameters for building the OpenGL context.
    let cb = glium::glutin::ContextBuilder::new().with_depth_buffer(24);
    // 4. Build the Display with the given window and OpenGL context parameters and register the
    //    window with the events_loop.
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let terrain = get_heightmap();

    let mut vertices =
        Vec::with_capacity((terrain.width() - 1) as usize * (terrain.height() - 1) as usize * 6);

    for x in 0..terrain.width() - 1 {
        for y in 0..terrain.height() - 1 {
            let id = terrain.index((x, y)) as u32;
            vertices.push(Vertex3 {
                position: [x as f32, y as f32, terrain[(x, y)]],
                id,
            });
            vertices.push(Vertex3 {
                position: [(x + 1) as f32, (y + 1) as f32, terrain[(x + 1, y + 1)]],
                id,
            });
            vertices.push(Vertex3 {
                position: [(x + 1) as f32, y as f32, terrain[(x + 1, y)]],
                id,
            });
            vertices.push(Vertex3 {
                position: [x as f32, y as f32, terrain[(x, y)]],
                id,
            });
            vertices.push(Vertex3 {
                position: [x as f32, (y + 1) as f32, terrain[(x, y + 1)]],
                id,
            });
            vertices.push(Vertex3 {
                position: [(x + 1) as f32, (y + 1) as f32, terrain[(x + 1, y + 1)]],
                id,
            });
        }
    }

    let screen_quad = vec![
        Vertex2 {
            position: [-1.0, -1.0],
            tex_coords: [0.0, 0.0],
        },
        Vertex2 {
            position: [1.0, -1.0],
            tex_coords: [1.0, 0.0],
        },
        Vertex2 {
            position: [-1.0, 1.0],
            tex_coords: [0.0, 1.0],
        },
        Vertex2 {
            position: [-1.0, 1.0],
            tex_coords: [0.0, 1.0],
        },
        Vertex2 {
            position: [1.0, -1.0],
            tex_coords: [1.0, 0.0],
        },
        Vertex2 {
            position: [1.0, 1.0],
            tex_coords: [1.0, 1.0],
        },
    ];

    let vertex_buffer = glium::VertexBuffer::new(&display, &vertices).unwrap();
    let screen_quad_buffer = glium::VertexBuffer::new(&display, &screen_quad).unwrap();

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
            #version 330

            in vec3 position;
            in uint id;
            out float height;
            flat out float id_in_float;

            uniform mat4 matrix;

            void main() {
                id_in_float = uintBitsToFloat(id);
                gl_Position = matrix * vec4(position.x, position.y, position.z * 32.0, 1.0);
            }
        "#;

    let fragment_shader_src = r#"
            #version 330

            in float height;
            flat in float id_in_float;
            out vec4 color;

            void main() {
                color = vec4(height, height, height, id_in_float);
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

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let screen_quad_program = glium::Program::from_source(
        &display,
        screen_vertex_shader_src,
        fragment_vertex_shader_src,
        None,
    )
    .unwrap();

    let mut attachments: Option<(
        glium::texture::Texture2d,
        glium::framebuffer::DepthRenderBuffer,
    )> = None;
    let pbo: glium::texture::pixel_buffer::PixelBuffer<(f32, f32, f32, f32)> =
        glium::texture::pixel_buffer::PixelBuffer::new_empty(&display, 1);

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let mut t: f32 = -0.5;
    let mut i: f32 = 0.002;

    event_loop.run(move |event, _, control_flow| {
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        t += i;
        if !(-0.5..=0.5).contains(&t) {
            i *= -1.0;
        }
        let scale = 1.0 / 128.0;
        let yaw = PI / 4.0;
        let pitch = PI / 4.0;
        let yc = yaw.cos();
        let ys = yaw.sin();
        let pc = pitch.cos();
        let ps = pitch.sin();
        // na::Matrix4::from_vec(vec![
        //     yc, ys, 0.0, 0.0,
        //     -ys * pc, yc * pc, ps, 0.0,
        //     0.0, 0.0, -1.0, 0.0,
        //     0.0, 0.0, 0.0, 1.0,
        // ])
        // .transpose()
        println!(
            "{:?}",
            pbo.read().map(|d| d[0]).unwrap_or((0.0, 0.0, 0.0, 0.0))
        );

        let mut target = display.draw();

        //update attachments
        if attachments.is_none()
            || (
                attachments.as_ref().unwrap().0.get_width(),
                attachments.as_ref().unwrap().0.get_height().unwrap(),
            ) != target.get_dimensions()
        {
            let (width, height) = target.get_dimensions();
            attachments = Some((
                glium::texture::Texture2d::empty_with_format(
                    &display,
                    glium::texture::UncompressedFloatFormat::F32F32F32F32,
                    glium::texture::MipmapsOption::NoMipmap,
                    width,
                    height,
                )
                .unwrap(),
                glium::framebuffer::DepthRenderBuffer::new(
                    &display,
                    glium::texture::DepthFormat::F32,
                    width,
                    height,
                )
                .unwrap(),
            ))
        }

        if let Some((ref texture, ref buffer)) = attachments {
            texture
                .main_level()
                .first_layer()
                .into_image(None)
                .unwrap()
                .raw_clear_buffer([0.0f32, 0.0, 0.0, 0.0]);

            let uniforms = uniform! {
            matrix: [
                [scale * yc, scale * -ys * pc, 0.0, 0.0],
                [scale * ys, scale * yc * pc, 0.0, 0.0],
                [0.0, scale * ps, scale * -1.0, 0.0],
                [-1.0, -0.0, 0.0, 1.0]
                // [yc, ys, 0.0, 0.0],
                // [-ys * pc, yc * pc, ps, 0.0],
                // [0.0, 0.0, -1.0, 0.0],
                // [0.0, 0.0, 0.0, 1.0],
            ]};
            let mut render_target =
                glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(&display, texture, buffer)
                    .unwrap();
            render_target.clear_depth(1.0);
            render_target
                .draw(&vertex_buffer, &indices, &program, &uniforms, &params)
                .unwrap();

            let uniforms = uniform! {
                tex: texture
            };

            target
                .draw(
                    &screen_quad_buffer,
                    &indices,
                    &screen_quad_program,
                    &uniforms,
                    &Default::default(),
                )
                .unwrap();
        }

        target.finish().unwrap();

        // committing into the picking pbo
        if let (Some(&(ref texture, _))) = (attachments.as_ref()) {
            let read_target = glium::Rect {
                left: 512,
                bottom: 512,
                width: 1,
                height: 1,
            };

            if read_target.left < texture.get_width()
                && read_target.bottom < texture.get_height().unwrap()
            {
                texture
                    .main_level()
                    .first_layer()
                    .into_image(None)
                    .unwrap()
                    .raw_read_to_pixel_buffer(&read_target, &pbo);
            } else {
                pbo.write(&[(0.0, 0.0, 0.0, 0.0)]);
            }
        } else {
            pbo.write(&[(0.0, 0.0, 0.0, 0.0)]);
        }
    });
}

fn get_heightmap() -> Grid<f32> {
    let power = 8;
    let weights = (0..power + 1)
        .map(|i| 1.0f32 / 1.125f32.powf((power - i) as f32))
        .collect::<Vec<_>>();
    let rises = simplex_noise(power, 1987, &weights)
        .normalize()
        .map(|_, z| (0.5 - z).abs() / 0.5);

    heightmap_from_rises_with_valleys(
        &rises,
        ValleyParameters {
            height_threshold: 0.25,
            rain_threshold: 128,
            rise: 0.01,
            origin_fn: |xy| rises.is_border(xy),
        },
    )
}
