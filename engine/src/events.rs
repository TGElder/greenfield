use commons::geometry::{Rectangle, XY};

use crate::engine::Engine;
use crate::graphics::Graphics;

pub trait EventHandler {
    fn handle(&mut self, event: &Event, engine: &mut dyn Engine, graphics: &mut dyn Graphics);
}

#[derive(PartialEq)]
pub enum ButtonState {
    Pressed,
    Released,
}

#[derive(PartialEq)]

pub enum Button {
    Mouse(MouseButton),
    Keyboard(KeyboardKey),
}

#[derive(PartialEq)]

pub enum MouseButton {
    Left,
    Middle,
    Right,
    WheelUp,
    WheelDown,
    Unknown,
}

#[derive(PartialEq)]
pub enum KeyboardKey {
    String(String),

    Backspace,
    Escape,

    Unknown,
}

impl From<&str> for KeyboardKey {
    fn from(value: &str) -> Self {
        KeyboardKey::String(value.into())
    }
}

pub enum Event {
    Init,
    Tick,
    Button { button: Button, state: ButtonState },
    MouseMoved(XY<u32>),
    WindowResize(Rectangle<u32>),
}
