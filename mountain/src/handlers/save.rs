use std::fs::File;
use std::io::BufWriter;

use engine::binding::Binding;

use crate::Components;

pub struct Handler {
    pub binding: Binding,
}

impl Handler {
    pub fn handle(&self, event: &engine::events::Event, components: &mut Components) {
        if !self.binding.binds_event(event) {
            return;
        }
        let mut file = BufWriter::new(File::create("default.save").unwrap());
        let speed = components.services.clock.speed();
        components.services.clock.set_speed(0.0);
        bincode::serialize_into(&mut file, components).unwrap();
        components.services.clock.set_speed(speed);
    }
}
