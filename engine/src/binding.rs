use crate::events::{Button, ButtonState, Event};

pub enum Binding {
    Single { button: Button, state: ButtonState },
    Multi(Vec<Binding>),
}

impl Binding {
    pub fn binds_event(&self, event: &Event) -> bool {
        match event {
            Event::Button { button, state } => self.binds(button, state),
            _ => false,
        }
    }

    fn binds(&self, other_button: &Button, other_state: &ButtonState) -> bool {
        match self {
            Binding::Single { button, state } => button == other_button && state == other_state,
            Binding::Multi(bindings) => bindings
                .iter()
                .any(|binding| binding.binds(other_button, other_state)),
        }
    }
}
