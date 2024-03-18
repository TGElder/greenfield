use engine::binding::Binding;

use crate::systems::terrain_artist;

pub struct Handler {
    pub binding: Binding,
}

impl Handler {
    pub fn handle(&mut self, event: &engine::events::Event, overlay: &mut terrain_artist::System) {
        if !self.binding.binds_event(event) {
            return;
        }

        overlay.toggle_show_pistes();
        overlay.update_all();
    }
}
