use engine::binding::Binding;

use crate::systems::skier_colors;
use crate::systems::skier_colors::Mode;

pub struct Handler {
    pub binding: Binding,
}

impl Handler {
    pub fn handle(
        &mut self,
        event: &engine::events::Event,
        skier_colors: &mut skier_colors::System,
    ) {
        if !self.binding.binds_event(event) {
            return;
        }

        match skier_colors.mode {
            Mode::Clothes => skier_colors.mode = Mode::Ability,
            Mode::Ability => skier_colors.mode = Mode::Clothes,
        }
    }
}
