use std::time::Duration;

use glium::glutin;

use crate::engine::Engine;
use crate::events::{Event, EventHandler};
use crate::glium_backend::graphics::{self, Graphics};
use crate::graphics::GraphicsBackend;

pub struct GameLoop<E, G> {
    event_loop: glutin::event_loop::EventLoop<()>,
    event_handler: E,
    graphics: G,
    state: GameState,
    parameters: Parameters,
}

struct GameState {
    running: bool,
}

impl Engine for GameState {
    fn shutdown(&mut self) {
        self.running = false;
    }
}

pub struct Parameters {
    pub frame_duration: Duration,
}

impl<E> GameLoop<E, Graphics>
where
    E: EventHandler + 'static,
{
    pub fn new(
        event_handler: E,
        parameters: Parameters,
        graphics_parameters: graphics::Parameters,
    ) -> GameLoop<E, Graphics> {
        let event_loop = glutin::event_loop::EventLoop::new();
        GameLoop {
            graphics: Graphics::from_event_loop(graphics_parameters, &event_loop).unwrap(), //TODO
            event_loop,
            event_handler,
            state: GameState { running: true },
            parameters,
        }
    }

    pub fn run(mut self) {
        self.event_loop.run(move |event, _, control_flow| {
            match event {
                glutin::event::Event::WindowEvent {
                    event: glutin::event::WindowEvent::CloseRequested,
                    ..
                } => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                glutin::event::Event::NewEvents(cause) => match cause {
                    glutin::event::StartCause::ResumeTimeReached { .. } => (),
                    glutin::event::StartCause::Init => (),
                    _ => return,
                },
                _ => return,
            }
            let next_frame_time = std::time::Instant::now() + self.parameters.frame_duration;
            *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

            self.event_handler
                .handle(&Event::Tick, &mut self.state, &mut self.graphics);

            match self.graphics.render() {
                Ok(_) => (),
                Err(err) => println!("Failed to render frame: {:?}", err),
            };
        });
    }
}
