use std::collections::{HashMap, HashSet};

use crate::model::open;
use crate::systems::messenger;

pub fn run(
    targets: &HashMap<usize, usize>,
    locations: &HashMap<usize, usize>,
    children: &HashMap<usize, Vec<usize>>,
    open: &mut HashMap<usize, open::Status>,
    messenger: &mut messenger::System,
) {
    let targeted = targets.values().collect::<HashSet<_>>();
    let populated = locations.values().collect::<HashSet<_>>();

    for (id, status) in open
        .iter_mut()
        .filter(|(_, &mut status)| status == open::Status::Closing)
    {
        if can_be_closed(id, &targeted, &populated, children) {
            *status = open::Status::Closed;
            messenger.send(format!("{} is now closed", id));
        }
    }
}

fn can_be_closed(
    id: &usize,
    targeted: &HashSet<&usize>,
    populated: &HashSet<&usize>,
    children: &HashMap<usize, Vec<usize>>,
) -> bool {
    if targeted.contains(id) {
        return false;
    }

    if populated.contains(id) {
        return false;
    }

    if let Some(child_ids) = children.get(id) {
        for child_id in child_ids {
            if !can_be_closed(child_id, targeted, populated, children) {
                return false;
            }
        }
    }

    true
}
