use std::fs::File;
use std::io::BufWriter;

use crate::systems::messenger;
use crate::Components;

pub fn trigger(
    components: &mut Components,
    save_file: &str,
    save_directory: &str,
    save_extension: &str,
    messenger: &mut messenger::System,
) {
    messenger.send(format!("Saving game to {}", save_file));

    let file = match File::create(format!(
        "{}{}.{}",
        save_directory, save_file, save_extension
    )) {
        Ok(file) => file,
        Err(e) => {
            let message = "Could not save game";
            messenger.send(message);
            eprintln!("{}: {}", message, e);
            return;
        }
    };

    let mut writer = BufWriter::new(file);
    let speed = components.services.clock.speed();
    components.services.clock.set_speed(0.0);

    if let Err(e) = bincode::serialize_into(&mut writer, components) {
        let message = "Could not save game";
        messenger.send(message);
        eprintln!("{}: {}", message, e);
        return;
    };

    components.services.clock.set_speed(speed);

    messenger.send(format!("Saved game to {}", save_file));
}
