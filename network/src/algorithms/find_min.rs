use core::hash::Hash;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::Debug;

use commons::unsafe_float_ordering;

use crate::model::{Edge, Network};

struct Node<T> {
    location: T,
    entrance: Option<Edge<T>>,
    edges: u32,
    distance: f32,
}

impl<T> Ord for Node<T>
where
    T: Eq,
{
    fn cmp(&self, other: &Node<T>) -> Ordering {
        unsafe_float_ordering(&self.distance, &other.distance).reverse()
    }
}

impl<T> PartialOrd for Node<T>
where
    T: Eq,
{
    fn partial_cmp(&self, other: &Node<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> PartialEq for Node<T>
where
    T: Eq,
{
    fn eq(&self, other: &Node<T>) -> bool {
        self.location == other.location && self.entrance == other.entrance
    }
}

impl<T> Eq for Node<T> where T: Eq {}

pub trait MinPath<T> {
    fn min_path(
        &self,
        from: T,
        max_edges: u32,
        distance_fn: &dyn Fn(&T, &Self) -> f32,
    ) -> Option<Vec<Edge<T>>>;
}

impl<T, N> MinPath<T> for N
where
    T: Copy + Debug + Eq + Hash,
    N: Network<T>,
{
    fn min_path(
        &self,
        from: T,
        max_edges: u32,
        distance_fn: &dyn Fn(&T, &Self) -> f32,
    ) -> Option<Vec<Edge<T>>> {
        let mut closed = HashSet::new();
        let mut entrances = HashMap::new();
        let mut heap = BinaryHeap::new();

        heap.push(Node {
            location: from,
            entrance: None,
            edges: 0,
            distance: distance_fn(&from, self),
        });

        while let Some(Node {
            location,
            entrance,
            edges,
            ..
        }) = heap.pop()
        {
            if closed.contains(&location) {
                continue;
            }
            closed.insert(location);

            if let Some(entrance) = entrance {
                entrances.insert(location, entrance);
            }

            if edges == max_edges {
                return Some(get_path(&from, &location, &mut entrances));
            }

            for edge in self.edges(&location) {
                let to = edge.to;
                if closed.contains(&to) {
                    continue;
                }
                let edges = edges + 1;
                heap.push(Node {
                    location: to,
                    entrance: Some(edge),
                    edges,
                    distance: distance_fn(&location, self),
                });
            }
        }

        None
    }
}

fn get_path<T>(from: &T, focus: &T, entrances: &mut HashMap<T, Edge<T>>) -> Vec<Edge<T>>
where
    T: Copy + Eq + Hash,
{
    let mut out = vec![];
    let mut focus = *focus;
    while *from != focus {
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
