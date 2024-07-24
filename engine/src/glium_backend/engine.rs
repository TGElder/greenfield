use std::error::Error;
use std::time::Duration;

use commons::geometry::{xy, Rectangle};
use winit::keyboard::{KeyCode, PhysicalKey};

use crate::engine::errors::InitializationError;
use crate::engine::Engine;
use crate::events::{Button, ButtonState, Event, EventHandler, KeyboardKey, MouseButton};
use crate::glium_backend::graphics::{self, GliumGraphics};
use crate::graphics::Graphics;

pub struct GliumEngine<E, G> {
    event_loop: winit::event_loop::EventLoop<()>,
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
        let event_loop = winit::event_loop::EventLoopBuilder::new().build().unwrap();
        Ok(GliumEngine {
            graphics: GliumGraphics::new(graphics_parameters, &event_loop)?,
            event_loop,
            event_handler,
            state: State { running: true },
            parameters,
        })
    }

    pub fn run(mut self) {
        self.event_handler
            .handle(&Event::Init, &mut self.state, &mut self.graphics);

        let mut cursor_position: Option<winit::dpi::PhysicalPosition<f64>> = None;

        self.event_loop
            .run(move |event, window_target| match event {
                winit::event::Event::NewEvents(cause) => match cause {
                    winit::event::StartCause::Init
                    | winit::event::StartCause::ResumeTimeReached { .. } => {
                        let next_frame_time =
                            std::time::Instant::now() + self.parameters.frame_duration;
                        window_target.set_control_flow(winit::event_loop::ControlFlow::WaitUntil(
                            next_frame_time,
                        ));

                        if let Some(position) = cursor_position {
                            let (x, y) = position.into();
                            self.event_handler.handle(
                                &Event::MouseMoved(xy(x, y)),
                                &mut self.state,
                                &mut self.graphics,
                            );
                        }

                        self.event_handler.handle(
                            &Event::Tick,
                            &mut self.state,
                            &mut self.graphics,
                        );

                        match self.graphics.render() {
                            Ok(_) => (),
                            Err(err) => println!("Failed to render frame: {err}"),
                        };
                    }
                    _ => (),
                },
                winit::event::Event::WindowEvent { event, .. } => {
                    if self.graphics.handle_window_event(&event).consumed {
                        return;
                    }

                    match event {
                        winit::event::WindowEvent::CloseRequested => {
                            window_target.exit();
                        }
                        winit::event::WindowEvent::CursorMoved { position, .. } => {
                            cursor_position = Some(position);
                        }
                        winit::event::WindowEvent::MouseInput { button, state, .. } => {
                            self.event_handler.handle(
                                &Event::Button {
                                    button: button.into(),
                                    state: state.into(),
                                },
                                &mut self.state,
                                &mut self.graphics,
                            );
                        }
                        winit::event::WindowEvent::MouseWheel {
                            delta: winit::event::MouseScrollDelta::LineDelta(_, y),
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
                        winit::event::WindowEvent::KeyboardInput {
                            event:
                                winit::event::KeyEvent {
                                    physical_key: keycode,
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
                        }
                        winit::event::WindowEvent::Resized(winit::dpi::PhysicalSize {
                            width,
                            height,
                        }) => {
                            self.event_handler.handle(
                                &Event::WindowResize(Rectangle { width, height }),
                                &mut self.state,
                                &mut self.graphics,
                            );
                        }
                        _ => (),
                    }
                }
                _ => (),
            })
            .unwrap();
    }
}

impl From<winit::event::ElementState> for ButtonState {
    fn from(state: winit::event::ElementState) -> Self {
        match state {
            winit::event::ElementState::Pressed => ButtonState::Pressed,
            winit::event::ElementState::Released => ButtonState::Released,
        }
    }
}

impl From<winit::event::MouseButton> for Button {
    fn from(button: winit::event::MouseButton) -> Self {
        let mouse_button = match button {
            winit::event::MouseButton::Left => MouseButton::Left,
            winit::event::MouseButton::Right => MouseButton::Right,
            winit::event::MouseButton::Middle => MouseButton::Middle,
            _ => MouseButton::Unknown,
        };
        Button::Mouse(mouse_button)
    }
}

impl From<PhysicalKey> for Button {
    fn from(keycode: PhysicalKey) -> Self {
        let key = match keycode {
            PhysicalKey::Code(KeyCode::Digit1) => KeyboardKey::Key1,
            PhysicalKey::Code(KeyCode::Digit2) => KeyboardKey::Key2,
            PhysicalKey::Code(KeyCode::Digit3) => KeyboardKey::Key3,
            PhysicalKey::Code(KeyCode::Digit4) => KeyboardKey::Key4,
            PhysicalKey::Code(KeyCode::Digit5) => KeyboardKey::Key5,
            PhysicalKey::Code(KeyCode::Digit6) => KeyboardKey::Key6,
            PhysicalKey::Code(KeyCode::Digit7) => KeyboardKey::Key7,
            PhysicalKey::Code(KeyCode::Digit8) => KeyboardKey::Key8,
            PhysicalKey::Code(KeyCode::Digit9) => KeyboardKey::Key9,
            PhysicalKey::Code(KeyCode::Digit0) => KeyboardKey::Key0,
            PhysicalKey::Code(KeyCode::KeyA) => KeyboardKey::A,
            PhysicalKey::Code(KeyCode::KeyB) => KeyboardKey::B,
            PhysicalKey::Code(KeyCode::KeyC) => KeyboardKey::C,
            PhysicalKey::Code(KeyCode::KeyD) => KeyboardKey::D,
            PhysicalKey::Code(KeyCode::KeyE) => KeyboardKey::E,
            PhysicalKey::Code(KeyCode::KeyF) => KeyboardKey::F,
            PhysicalKey::Code(KeyCode::KeyG) => KeyboardKey::G,
            PhysicalKey::Code(KeyCode::KeyH) => KeyboardKey::H,
            PhysicalKey::Code(KeyCode::KeyI) => KeyboardKey::I,
            PhysicalKey::Code(KeyCode::KeyJ) => KeyboardKey::J,
            PhysicalKey::Code(KeyCode::KeyK) => KeyboardKey::K,
            PhysicalKey::Code(KeyCode::KeyL) => KeyboardKey::L,
            PhysicalKey::Code(KeyCode::KeyM) => KeyboardKey::M,
            PhysicalKey::Code(KeyCode::KeyN) => KeyboardKey::N,
            PhysicalKey::Code(KeyCode::KeyO) => KeyboardKey::O,
            PhysicalKey::Code(KeyCode::KeyP) => KeyboardKey::P,
            PhysicalKey::Code(KeyCode::KeyQ) => KeyboardKey::Q,
            PhysicalKey::Code(KeyCode::KeyR) => KeyboardKey::R,
            PhysicalKey::Code(KeyCode::KeyS) => KeyboardKey::S,
            PhysicalKey::Code(KeyCode::KeyT) => KeyboardKey::T,
            PhysicalKey::Code(KeyCode::KeyU) => KeyboardKey::U,
            PhysicalKey::Code(KeyCode::KeyV) => KeyboardKey::V,
            PhysicalKey::Code(KeyCode::KeyW) => KeyboardKey::W,
            PhysicalKey::Code(KeyCode::KeyX) => KeyboardKey::X,
            PhysicalKey::Code(KeyCode::KeyY) => KeyboardKey::Y,
            PhysicalKey::Code(KeyCode::KeyZ) => KeyboardKey::Z,
            PhysicalKey::Code(KeyCode::Equal) => KeyboardKey::Equal,
            PhysicalKey::Code(KeyCode::Minus) => KeyboardKey::Minus,
            PhysicalKey::Code(KeyCode::Comma) => KeyboardKey::Comma,
            PhysicalKey::Code(KeyCode::Period) => KeyboardKey::Period,
            PhysicalKey::Code(KeyCode::BracketLeft) => KeyboardKey::BracketLeft,
            PhysicalKey::Code(KeyCode::BracketRight) => KeyboardKey::BracketRight,
            PhysicalKey::Code(KeyCode::Slash) => KeyboardKey::Slash,
            PhysicalKey::Code(KeyCode::Backslash) => KeyboardKey::Backslash,
            _ => KeyboardKey::Unknown,
        };
        Button::Keyboard(key)
    }
}
