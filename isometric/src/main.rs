extern crate glium;

mod glium_backend;
mod graphics;

use commons::color::Color;
use commons::grid::Grid;
use commons::noise::simplex_noise;
use glium::glutin;
use terrain_gen::with_valleys::{heightmap_from_rises_with_valleys, ValleyParameters};

use crate::glium_backend::graphics::Graphics;
use crate::graphics::elements::Triangle;
use crate::graphics::GraphicsBackend;

fn main() {
    // 1. The **winit::EventsLoop** for handling events.
    let event_loop = glutin::event_loop::EventLoop::new();
    // 2. Parameters for building the Window.
    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0))
        .with_title("Hello world");
    // 3. Parameters for building the OpenGL context.
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    // 4. Build the Display with the given window and OpenGL context parameters and register the
    //    window with the events_loop.
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    let mut graphics = Graphics::new(display);

    let terrain = get_heightmap().map(|_, z| z * 32.0);

    let mut triangles =
        Vec::with_capacity((terrain.width() - 1) as usize * (terrain.height() - 1) as usize * 2);
    for x in 0..terrain.width() - 1 {
        for y in 0..terrain.height() - 1 {
            let id = terrain.index((x, y)) as u32;
            let z = terrain[(x, y)];
            let corners = [(0, 0), (1, 0), (1, 1), (0, 1)]
                .iter()
                .map(|(dx, dy)| [(x + dx) as f32, (y + dy) as f32, terrain[(x + dx, y + dy)]])
                .collect::<Vec<_>>();
            triangles.push(Triangle {
                id,
                corners: [corners[0], corners[2], corners[1]],
                color: Color::rgb(z, z, z),
            });
            triangles.push(Triangle {
                id,
                corners: [corners[0], corners[3], corners[2]],
                color: Color::rgb(z, z, z),
            });
        }
    }

    graphics.draw_triangles(&triangles);

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

        graphics.render();
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
