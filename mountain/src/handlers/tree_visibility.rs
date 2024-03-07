use engine::binding::Binding;

use crate::systems::tree_artist;

pub struct Handler {
    pub binding: Binding,
}

impl Handler {
    pub fn handle(
        &self,
        event: &engine::events::Event,
        tree_artist: &mut tree_artist::System,
        graphics: &mut dyn engine::graphics::Graphics,
    ) {
        if !self.binding.binds_event(event) {
            return;
        }
        tree_artist.toggle_visible(graphics);
    }
}
