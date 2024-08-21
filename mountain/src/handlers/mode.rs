use crate::services::mode::Mode;
use crate::{services, Bindings};

pub fn handle(
    event: &engine::events::Event,
    bindings: &Bindings,
    service: &mut services::mode::Service,
) {
    for (&mode, binding) in bindings.mode.iter() {
        if binding.binds_event(event) {
            if service.mode() == mode {
                service.set_mode(Mode::None);
            } else {
                service.set_mode(mode);
            }
            return;
        }
    }
}
