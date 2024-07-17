use std::error::Error;
use std::time::Duration;

use commons::geometry::{xy, Rectangle};

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
                winit::event::Event::WindowEvent { event, .. } => match event {
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
                },
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

impl From<winit::keyboard::PhysicalKey> for Button {
    fn from(keycode: winit::keyboard::PhysicalKey) -> Self {
        let key = match keycode {
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Digit1) => {
                KeyboardKey::Key1
            }
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Digit2) => {
                KeyboardKey::Key2
            }
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Digit3) => {
                KeyboardKey::Key3
            }
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Digit4) => {
                KeyboardKey::Key4
            }
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Digit5) => {
                KeyboardKey::Key5
            }
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Digit6) => {
                KeyboardKey::Key6
            }
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Digit7) => {
                KeyboardKey::Key7
            }
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Digit8) => {
                KeyboardKey::Key8
            }
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Digit9) => {
                KeyboardKey::Key9
            }
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Digit0) => {
                KeyboardKey::Key0
            }
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyA) => KeyboardKey::A,
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyB) => KeyboardKey::B,
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyC) => KeyboardKey::C,
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyD) => KeyboardKey::D,
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyE) => KeyboardKey::E,
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyF) => KeyboardKey::F,
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyG) => KeyboardKey::G,
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyH) => KeyboardKey::H,
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyI) => KeyboardKey::I,
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyJ) => KeyboardKey::J,
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyK) => KeyboardKey::K,
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyL) => KeyboardKey::L,
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyM) => KeyboardKey::M,
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyN) => KeyboardKey::N,
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyO) => KeyboardKey::O,
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyP) => KeyboardKey::P,
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyQ) => KeyboardKey::Q,
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyR) => KeyboardKey::R,
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyS) => KeyboardKey::S,
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyT) => KeyboardKey::T,
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyU) => KeyboardKey::U,
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyV) => KeyboardKey::V,
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyW) => KeyboardKey::W,
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyX) => KeyboardKey::X,
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyY) => KeyboardKey::Y,
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyZ) => KeyboardKey::Z,
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Equal) => {
                KeyboardKey::Equal
            }
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Minus) => {
                KeyboardKey::Minus
            }
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Comma) => {
                KeyboardKey::Comma
            }
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Period) => {
                KeyboardKey::Period
            }
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::BracketLeft) => {
                KeyboardKey::BracketLeft
            }
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::BracketRight) => {
                KeyboardKey::BracketRight
            }
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Slash) => {
                KeyboardKey::Slash
            }
            winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Backslash) => {
                KeyboardKey::Backslash
            }
            _ => KeyboardKey::Unknown,
        };
        Button::Keyboard(key)
    }
}
