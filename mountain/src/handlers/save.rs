use std::fs::File;
use std::io::BufWriter;

use engine::binding::Binding;

use crate::Components;

pub struct Handler {
    pub binding: Binding,
}

impl Handler {
    pub fn handle(&self, event: &engine::events::Event, components: &Components) {
        if !self.binding.binds_event(event) {
            return;
        }
        let mut file = BufWriter::new(File::create("default.save").unwrap());
        bincode::serialize_into(&mut file, components).unwrap();
    }
}
