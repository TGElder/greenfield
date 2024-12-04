use std::collections::HashMap;

use commons::map::ContainsKeyValue;

use crate::model::open;

pub fn run(open: &HashMap<usize, open::Status>, targets: &mut HashMap<usize, usize>) {
    for (id, target_id) in targets.iter() {
        if open.contains_key_value(target_id, open::Status::Closed) {
            eprintln!("{} has closed target {}", id, target_id);
        }
    }
}
