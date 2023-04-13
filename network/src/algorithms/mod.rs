use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use crate::model::Edge;

pub mod find_best_within_budget;
pub mod find_path;

fn get_path<T>(from: &HashSet<T>, focus: &T, entrances: &mut HashMap<T, Edge<T>>) -> Vec<Edge<T>>
where
    T: Copy + Eq + Hash,
{
    let mut out = vec![];
    let mut focus = *focus;
    while !from.contains(&focus) {
        let entrance = entrances.remove(&focus);
        match entrance {
            Some(entrance) => {
                focus = entrance.from;
                out.push(entrance);
            }
            None => panic!("!"),
        }
    }
    out.reverse();
    out
}
