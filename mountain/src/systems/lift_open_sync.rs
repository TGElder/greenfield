use std::collections::HashMap;

use commons::map::ContainsKeyValue;

use crate::model::lift::Lift;

pub fn run(lifts: &HashMap<usize, Lift>, open: &mut HashMap<usize, bool>) {
    for (lift_id, lift) in lifts.iter() {
        let is_open = open.contains_key_value(lift_id, true);
        open.insert(lift.pick_up.id, is_open);
        open.insert(lift.drop_off.id, is_open);
    }
}
