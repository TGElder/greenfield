use std::error::Error;
use std::time::Duration;

use commons::geometry::{xy, Rectangle};
use glium::glutin;
use glium::glutin::event::MouseScrollDelta;

use crate::engine::errors::InitializationError;
use crate::engine::Engine;
use crate::events::{Button, ButtonState, Event, EventHandler, KeyboardKey, MouseButton};
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
                                &Event::Button {
                                    button: Button::Mouse(MouseButton::WheelUp),
                                    state: ButtonState::Pressed,
                                },
                                &mut self.state,
                                &mut self.graphics,
                            );
                        } else if y < 0.0 {
                            self.event_handler.handle(
                                &Event::Button {
                                    button: Button::Mouse(MouseButton::WheelDown),
                                    state: ButtonState::Pressed,
                                },
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
        let mouse_button = match button {
            glutin::event::MouseButton::Left => MouseButton::Left,
            glutin::event::MouseButton::Right => MouseButton::Right,
            glutin::event::MouseButton::Middle => MouseButton::Middle,
            glutin::event::MouseButton::Other(_) => MouseButton::Unknown,
        };
        Button::Mouse(mouse_button)
    }
}

impl From<glutin::event::VirtualKeyCode> for Button {
    fn from(keycode: glutin::event::VirtualKeyCode) -> Self {
        let key = match keycode {
            glutin::event::VirtualKeyCode::Key1 => KeyboardKey::Key1,
            glutin::event::VirtualKeyCode::Key2 => KeyboardKey::Key2,
            glutin::event::VirtualKeyCode::Key3 => KeyboardKey::Key3,
            glutin::event::VirtualKeyCode::Key4 => KeyboardKey::Key4,
            glutin::event::VirtualKeyCode::Key5 => KeyboardKey::Key5,
            glutin::event::VirtualKeyCode::Key6 => KeyboardKey::Key6,
            glutin::event::VirtualKeyCode::Key7 => KeyboardKey::Key7,
            glutin::event::VirtualKeyCode::Key8 => KeyboardKey::Key8,
            glutin::event::VirtualKeyCode::Key9 => KeyboardKey::Key9,
            glutin::event::VirtualKeyCode::Key0 => KeyboardKey::Key0,
            glutin::event::VirtualKeyCode::A => KeyboardKey::A,
            glutin::event::VirtualKeyCode::B => KeyboardKey::B,
            glutin::event::VirtualKeyCode::C => KeyboardKey::C,
            glutin::event::VirtualKeyCode::D => KeyboardKey::D,
            glutin::event::VirtualKeyCode::E => KeyboardKey::E,
            glutin::event::VirtualKeyCode::F => KeyboardKey::F,
            glutin::event::VirtualKeyCode::G => KeyboardKey::G,
            glutin::event::VirtualKeyCode::H => KeyboardKey::H,
            glutin::event::VirtualKeyCode::I => KeyboardKey::I,
            glutin::event::VirtualKeyCode::J => KeyboardKey::J,
            glutin::event::VirtualKeyCode::K => KeyboardKey::K,
            glutin::event::VirtualKeyCode::L => KeyboardKey::L,
            glutin::event::VirtualKeyCode::M => KeyboardKey::M,
            glutin::event::VirtualKeyCode::N => KeyboardKey::N,
            glutin::event::VirtualKeyCode::O => KeyboardKey::O,
            glutin::event::VirtualKeyCode::P => KeyboardKey::P,
            glutin::event::VirtualKeyCode::Q => KeyboardKey::Q,
            glutin::event::VirtualKeyCode::R => KeyboardKey::R,
            glutin::event::VirtualKeyCode::S => KeyboardKey::S,
            glutin::event::VirtualKeyCode::T => KeyboardKey::T,
            glutin::event::VirtualKeyCode::U => KeyboardKey::U,
            glutin::event::VirtualKeyCode::V => KeyboardKey::V,
            glutin::event::VirtualKeyCode::W => KeyboardKey::W,
            glutin::event::VirtualKeyCode::X => KeyboardKey::X,
            glutin::event::VirtualKeyCode::Y => KeyboardKey::Y,
            glutin::event::VirtualKeyCode::Z => KeyboardKey::Z,
            glutin::event::VirtualKeyCode::Plus => KeyboardKey::Plus,
            glutin::event::VirtualKeyCode::Minus => KeyboardKey::Minus,
            glutin::event::VirtualKeyCode::Comma => KeyboardKey::Comma,
            glutin::event::VirtualKeyCode::Period => KeyboardKey::Period,
            _ => KeyboardKey::Unknown,
        };
        Button::Keyboard(key)
    }
}
