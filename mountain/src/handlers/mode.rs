use crate::model::selection::Selection;
use crate::{services, Bindings};

pub fn handle(
    event: &engine::events::Event,
    bindings: &Bindings,
    service: &mut services::mode::Service,
    selection: &mut Selection,
) {
    for (&mode, binding) in bindings.mode.iter() {
        if binding.binds_event(event) {
            service.set_mode(mode, selection);
            return;
        }
    }
}
