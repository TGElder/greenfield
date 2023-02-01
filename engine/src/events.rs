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

pub enum Event {
    Tick,
    MouseMoved((u32, u32)),
    MouseInput {
        button: MouseButton,
        state: ButtonState,
    },
}
