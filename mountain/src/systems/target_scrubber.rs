use std::collections::HashMap;

use commons::map::ContainsKeyValue;

pub fn run(open: &HashMap<usize, bool>, targets: &mut HashMap<usize, usize>) {
    targets.retain(|_, target| open.contains_key_value(target, true))
}
