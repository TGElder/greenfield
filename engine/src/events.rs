use crate::engine::Engine;
use crate::graphics::Graphics;

pub trait EventHandler {
    fn handle(&mut self, event: &Event, engine: &mut dyn Engine, graphics: &mut dyn Graphics);
}

pub enum ButtonState {
    Pressed,
    Released,
}

pub enum MouseButton {
    Left,
    Middle,
    Right,
    Unknown,
}

pub enum MouseWheelDirection {
    Up,
    Down,
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

    Unknown,
}

pub enum Event {
    Tick,
    MouseMoved((u32, u32)),
    MouseInput {
        button: MouseButton,
        state: ButtonState,
    },
    MouseWheel(MouseWheelDirection),
    KeyboardInput {
        key: KeyboardKey,
        state: ButtonState,
    },
    WindowResize {
        width: u32,
        height: u32,
    },
}
