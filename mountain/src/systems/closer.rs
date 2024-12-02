use std::collections::{HashMap, HashSet};

use crate::model::open;
use crate::systems::messenger;

pub fn run(
    targets: &HashMap<usize, usize>,
    locations: &HashMap<usize, usize>,
    open: &mut HashMap<usize, open::Status>,
    messenger: &mut messenger::System,
) {
    let targeted = targets.values().collect::<HashSet<_>>();
    let populated = locations.values().collect::<HashSet<_>>();

    for (id, status) in open
        .iter_mut()
        .filter(|(_, &mut status)| status == open::Status::Closing)
    {
        if !targeted.contains(id) && !populated.contains(id) {
            *status = open::Status::Closed;
            messenger.send(format!("{} is now closed", id));
        }
    }
}
