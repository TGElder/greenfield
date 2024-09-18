use std::fs::File;
use std::io::BufWriter;

use crate::Components;

pub fn trigger(components: &mut Components) {
    let mut file = BufWriter::new(File::create("default.save").unwrap());
    let speed = components.services.clock.speed();
    components.services.clock.set_speed(0.0);
    bincode::serialize_into(&mut file, components).unwrap();
    components.services.clock.set_speed(speed);
}
