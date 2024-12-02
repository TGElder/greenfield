use std::collections::HashMap;

use commons::map::ContainsKeyValue;

use crate::model::open;

pub fn run(open: &HashMap<usize, open::Status>, targets: &mut HashMap<usize, usize>) {
    targets.retain(|_, target| open.contains_key_value(target, open::Status::Open))
}
