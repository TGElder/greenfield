use std::collections::{HashMap, HashSet};

pub fn run(open: &HashSet<usize>, targets: &mut HashMap<usize, usize>) {
    targets.retain(|_, target| open.contains(target))
}
