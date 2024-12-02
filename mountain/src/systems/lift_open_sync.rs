use std::collections::HashMap;

use crate::model::lift::Lift;
use crate::model::open;

pub fn run(lifts: &HashMap<usize, Lift>, open: &mut HashMap<usize, open::Status>) {
    for (lift_id, lift) in lifts.iter() {
        if let Some(&status) = open.get(lift_id) {
            open.insert(lift.pick_up.id, status);
            open.insert(lift.drop_off.id, status);
        }
    }
}
