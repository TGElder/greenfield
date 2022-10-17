use std::time::Duration;

use glium::glutin;

use crate::game::{self, Game};
use crate::graphics::GraphicsBackend;

pub struct GameLoop {
    pub(super) event_loop: glutin::event_loop::EventLoop<()>,
    parameters: Parameters,
}

pub struct Parameters {
    pub frame_duration: Duration,
}

impl GameLoop {
    pub fn new(parameters: Parameters) -> GameLoop {
        GameLoop {
            event_loop: glutin::event_loop::EventLoop::new(),
            parameters,
        }
    }

    pub fn run<GAME, GFX>(self, mut game: GAME, mut graphics: GFX)
    where
        GAME: Game + 'static,
        GFX: GraphicsBackend + 'static,
    {
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

            let game_state = game.update(&mut graphics);
            match graphics.render() {
                Ok(_) => (),
                Err(err) => println!("Failed to render {:?}", err),
            };

            if let game::State::Terminated = game_state {
                *control_flow = glutin::event_loop::ControlFlow::Exit
            }
        });
    }
}
