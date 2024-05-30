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
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key0,

    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    Plus,
    Minus,

    Comma,
    Period,

    BracketLeft,
    BracketRight,

    Unknown,
}

pub enum Event {
    Init,
    Tick,
    Button { button: Button, state: ButtonState },
    MouseMoved(XY<u32>),
    WindowResize(Rectangle<u32>),
}
