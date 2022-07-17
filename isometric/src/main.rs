extern crate glium;

use std::f32::consts::PI;

use commons::grid::Grid;
use commons::noise::simplex_noise;
use glium::{glutin, implement_vertex};
use glium::{uniform, Surface};
use terrain_gen::with_valleys::{heightmap_from_rises_with_valleys, ValleyParameters};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
}

implement_vertex!(Vertex, position);

fn main() {
    // 1. The **winit::EventsLoop** for handling events.
    let mut event_loop = glium::glutin::event_loop::EventLoop::new();
    // 2. Parameters for building the Window.
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(1024.0, 768.0))
        .with_title("Hello world");
    // 3. Parameters for building the OpenGL context.
    let cb = glium::glutin::ContextBuilder::new();
    // 4. Build the Display with the given window and OpenGL context parameters and register the
    //    window with the events_loop.
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let terrain = get_heightmap();

    let mut vertices =
        Vec::with_capacity((terrain.width() - 1) as usize * (terrain.height() - 1) as usize * 6);

    for x in 0..terrain.width() - 1 {
        for y in 0..terrain.height() - 1 {
            vertices.push(Vertex {
                position: [x as f32, y as f32, terrain[(x, y)]],
            });
            vertices.push(Vertex {
                position: [(x + 1) as f32, (y + 1) as f32, terrain[(x + 1, y + 1)]],
            });
            vertices.push(Vertex {
                position: [(x + 1) as f32, y as f32, terrain[(x + 1, y)]],
            });
            vertices.push(Vertex {
                position: [x as f32, y as f32, terrain[(x, y)]],
            });
            vertices.push(Vertex {
                position: [x as f32, (y + 1) as f32, terrain[(x, y + 1)]],
            });
            vertices.push(Vertex {
                position: [(x + 1) as f32, (y + 1) as f32, terrain[(x + 1, y + 1)]],
            });
        }
    }

    let vertex_buffer = glium::VertexBuffer::new(&display, &vertices).unwrap();

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
            #version 130

            in vec3 position;
            out float height;

            uniform mat4 matrix;

            void main() {
                height = position.z;
                gl_Position = matrix * vec4(position.x, position.y, position.z * 32.0, 1.0);
            }
        "#;

    let fragment_shader_src = r#"
            #version 130

            in float height;
            out vec4 color;

            void main() {
                color = vec4(height, height, height, 1.0);
            }
        "#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

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
            ]
        };

        let mut target = display.draw();

        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
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
