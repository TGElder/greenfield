use std::error::Error;
use std::time::Duration;

use commons::geometry::{xy, Rectangle};
use glium::glutin;
use glium::glutin::event::MouseScrollDelta;

use crate::engine::errors::InitializationError;
use crate::engine::Engine;
use crate::events::{
    Button, ButtonState, Event, EventHandler, KeyboardKey, MouseButton, MouseWheelDirection,
};
use crate::glium_backend::graphics::{self, GliumGraphics};
use crate::graphics::Graphics;

pub struct GliumEngine<E, G> {
    event_loop: glutin::event_loop::EventLoop<()>,
    event_handler: E,
    graphics: G,
    state: State,
    parameters: Parameters,
}

struct State {
    running: bool,
}

impl Engine for State {
    fn shutdown(&mut self) {
        self.running = false;
    }
}

pub struct Parameters {
    pub frame_duration: Duration,
}

impl<E> GliumEngine<E, GliumGraphics>
where
    E: EventHandler + 'static,
{
    pub fn new(
        event_handler: E,
        parameters: Parameters,
        graphics_parameters: graphics::Parameters,
    ) -> Result<GliumEngine<E, GliumGraphics>, InitializationError> {
        Ok(Self::new_unsafe(
            event_handler,
            parameters,
            graphics_parameters,
        )?)
    }

    fn new_unsafe(
        event_handler: E,
        parameters: Parameters,
        graphics_parameters: graphics::Parameters,
    ) -> Result<GliumEngine<E, GliumGraphics>, Box<dyn Error>> {
        let event_loop = glutin::event_loop::EventLoop::new();
        Ok(GliumEngine {
            graphics: GliumGraphics::headful(graphics_parameters, &event_loop)?,
            event_loop,
            event_handler,
            state: State { running: true },
            parameters,
        })
    }

    pub fn run(mut self) {
        self.event_handler
            .handle(&Event::Init, &mut self.state, &mut self.graphics);

        self.event_loop.run(move |event, _, control_flow| {
            match event {
                glutin::event::Event::NewEvents(cause) => match cause {
                    glutin::event::StartCause::ResumeTimeReached { .. } => (),
                    glutin::event::StartCause::Init => (),
                    _ => return,
                },
                glutin::event::Event::WindowEvent { event, .. } => match event {
                    glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        return;
                    }
                    glutin::event::WindowEvent::CursorMoved { position, .. } => {
                        let (x, y) = position.into();
                        self.event_handler.handle(
                            &Event::MouseMoved(xy(x, y)),
                            &mut self.state,
                            &mut self.graphics,
                        );
                        return;
                    }
                    glutin::event::WindowEvent::MouseInput { button, state, .. } => {
                        self.event_handler.handle(
                            &Event::Button {
                                button: button.into(),
                                state: state.into(),
                            },
                            &mut self.state,
                            &mut self.graphics,
                        );
                        return;
                    }
                    glutin::event::WindowEvent::MouseWheel {
                        delta: MouseScrollDelta::LineDelta(_, y),
                        ..
                    } => {
                        if y > 0.0 {
                            self.event_handler.handle(
                                &Event::MouseWheel(MouseWheelDirection::Up),
                                &mut self.state,
                                &mut self.graphics,
                            );
                        } else if y < 0.0 {
                            self.event_handler.handle(
                                &Event::MouseWheel(MouseWheelDirection::Down),
                                &mut self.state,
                                &mut self.graphics,
                            );
                        }
                    }
                    glutin::event::WindowEvent::KeyboardInput {
                        input:
                            glutin::event::KeyboardInput {
                                virtual_keycode: Some(keycode),
                                state,
                                ..
                            },
                        ..
                    } => {
                        self.event_handler.handle(
                            &Event::Button {
                                button: keycode.into(),
                                state: state.into(),
                            },
                            &mut self.state,
                            &mut self.graphics,
                        );
                        return;
                    }
                    glutin::event::WindowEvent::Resized(glutin::dpi::PhysicalSize {
                        width,
                        height,
                    }) => {
                        self.event_handler.handle(
                            &Event::WindowResize(Rectangle { width, height }),
                            &mut self.state,
                            &mut self.graphics,
                        );
                        return;
                    }
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
                Err(err) => println!("Failed to render frame: {err}"),
            };
        });
    }
}

impl From<glutin::event::ElementState> for ButtonState {
    fn from(state: glutin::event::ElementState) -> Self {
        match state {
            glutin::event::ElementState::Pressed => ButtonState::Pressed,
            glutin::event::ElementState::Released => ButtonState::Released,
        }
    }
}

impl From<glutin::event::MouseButton> for Button {
    fn from(button: glutin::event::MouseButton) -> Self {
        match button {
            glutin::event::MouseButton::Left => Button::Mouse(MouseButton::Left),
            glutin::event::MouseButton::Right => Button::Mouse(MouseButton::Right),
            glutin::event::MouseButton::Middle => Button::Mouse(MouseButton::Middle),
            glutin::event::MouseButton::Other(_) => Button::Mouse(MouseButton::Unknown),
        }
    }
}

impl From<glutin::event::VirtualKeyCode> for Button {
    fn from(keycode: glutin::event::VirtualKeyCode) -> Self {
        match keycode {
            glutin::event::VirtualKeyCode::Key1 => Button::Keyboard(KeyboardKey::Key1),
            glutin::event::VirtualKeyCode::Key2 => Button::Keyboard(KeyboardKey::Key2),
            glutin::event::VirtualKeyCode::Key3 => Button::Keyboard(KeyboardKey::Key3),
            glutin::event::VirtualKeyCode::Key4 => Button::Keyboard(KeyboardKey::Key4),
            glutin::event::VirtualKeyCode::Key5 => Button::Keyboard(KeyboardKey::Key5),
            glutin::event::VirtualKeyCode::Key6 => Button::Keyboard(KeyboardKey::Key6),
            glutin::event::VirtualKeyCode::Key7 => Button::Keyboard(KeyboardKey::Key7),
            glutin::event::VirtualKeyCode::Key8 => Button::Keyboard(KeyboardKey::Key8),
            glutin::event::VirtualKeyCode::Key9 => Button::Keyboard(KeyboardKey::Key9),
            glutin::event::VirtualKeyCode::Key0 => Button::Keyboard(KeyboardKey::Key0),
            glutin::event::VirtualKeyCode::A => Button::Keyboard(KeyboardKey::A),
            glutin::event::VirtualKeyCode::B => Button::Keyboard(KeyboardKey::B),
            glutin::event::VirtualKeyCode::C => Button::Keyboard(KeyboardKey::C),
            glutin::event::VirtualKeyCode::D => Button::Keyboard(KeyboardKey::D),
            glutin::event::VirtualKeyCode::E => Button::Keyboard(KeyboardKey::E),
            glutin::event::VirtualKeyCode::F => Button::Keyboard(KeyboardKey::F),
            glutin::event::VirtualKeyCode::G => Button::Keyboard(KeyboardKey::G),
            glutin::event::VirtualKeyCode::H => Button::Keyboard(KeyboardKey::H),
            glutin::event::VirtualKeyCode::I => Button::Keyboard(KeyboardKey::I),
            glutin::event::VirtualKeyCode::J => Button::Keyboard(KeyboardKey::J),
            glutin::event::VirtualKeyCode::K => Button::Keyboard(KeyboardKey::K),
            glutin::event::VirtualKeyCode::L => Button::Keyboard(KeyboardKey::L),
            glutin::event::VirtualKeyCode::M => Button::Keyboard(KeyboardKey::M),
            glutin::event::VirtualKeyCode::N => Button::Keyboard(KeyboardKey::N),
            glutin::event::VirtualKeyCode::O => Button::Keyboard(KeyboardKey::O),
            glutin::event::VirtualKeyCode::P => Button::Keyboard(KeyboardKey::P),
            glutin::event::VirtualKeyCode::Q => Button::Keyboard(KeyboardKey::Q),
            glutin::event::VirtualKeyCode::R => Button::Keyboard(KeyboardKey::R),
            glutin::event::VirtualKeyCode::S => Button::Keyboard(KeyboardKey::S),
            glutin::event::VirtualKeyCode::T => Button::Keyboard(KeyboardKey::T),
            glutin::event::VirtualKeyCode::U => Button::Keyboard(KeyboardKey::U),
            glutin::event::VirtualKeyCode::V => Button::Keyboard(KeyboardKey::V),
            glutin::event::VirtualKeyCode::W => Button::Keyboard(KeyboardKey::W),
            glutin::event::VirtualKeyCode::X => Button::Keyboard(KeyboardKey::X),
            glutin::event::VirtualKeyCode::Y => Button::Keyboard(KeyboardKey::Y),
            glutin::event::VirtualKeyCode::Z => Button::Keyboard(KeyboardKey::Z),
            glutin::event::VirtualKeyCode::Plus => Button::Keyboard(KeyboardKey::Plus),
            glutin::event::VirtualKeyCode::Minus => Button::Keyboard(KeyboardKey::Minus),
            _ => Button::Keyboard(KeyboardKey::Unknown),
        }
    }
}
