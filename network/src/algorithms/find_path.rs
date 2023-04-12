use core::hash::Hash;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::model::{Edge, Network};

#[derive(Eq)]
struct Node<T> {
    location: T,
    entrance: Option<Edge<T>>,
    distance_from_start: u64,
    estimated_distance_via_this_node: u64,
}

impl<T> Ord for Node<T>
where
    T: Eq,
{
    fn cmp(&self, other: &Node<T>) -> Ordering {
        self.estimated_distance_via_this_node
            .cmp(&other.estimated_distance_via_this_node)
            .reverse()
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
        self.estimated_distance_via_this_node == other.estimated_distance_via_this_node
    }
}

pub trait FindPath<T> {
    fn find_path(
        &self,
        from: HashSet<T>,
        to: HashSet<T>,
        heuristic: &dyn Fn(&T) -> u64,
    ) -> Option<Vec<Edge<T>>>;
}

impl<T, N> FindPath<T> for N
where
    T: Copy + Eq + Hash,
    N: Network<T>,
{
    fn find_path(
        &self,
        from: HashSet<T>,
        to: HashSet<T>,
        heuristic: &dyn Fn(&T) -> u64,
    ) -> Option<Vec<Edge<T>>> {
        let mut closed = HashSet::new();
        let mut entrances = HashMap::new();
        let mut heap = BinaryHeap::new();

        for from in from.iter() {
            heap.push(Node {
                location: *from,
                entrance: None,
                distance_from_start: 0,
                estimated_distance_via_this_node: heuristic(from),
            });
        }

        while let Some(Node {
            location,
            entrance,
            distance_from_start,
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

            if to.contains(&location) {
                return Some(get_path(&from, &location, &mut entrances));
            }

            for edge in self.edges(&location) {
                let to = edge.to;
                if closed.contains(&to) {
                    continue;
                }
                let distance_from_start = distance_from_start + edge.cost as u64;
                heap.push(Node {
                    location: to,
                    entrance: Some(edge),
                    distance_from_start,
                    estimated_distance_via_this_node: distance_from_start + heuristic(&location),
                });
            }
        }

        None
    }
}

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

#[cfg(test)]
mod tests {
    use std::iter;

    use maplit::hashset;

    use crate::model::Network;

    use super::*;

    #[test]
    fn path_is_possible() {
        // given
        //
        // [4] <-1-> [0] <-0-> [1] <-3-> [2] <-1-> [3]
        //            ^___________________^
        //                      2

        struct TestNetwork {}

        impl Network<usize> for TestNetwork {
            fn edges<'a>(&'a self, from: &'a usize) -> Box<dyn Iterator<Item = Edge<usize>> + 'a> {
                match from {
                    0 => Box::new(
                        [
                            Edge {
                                from: 0,
                                to: 1,
                                cost: 0,
                            },
                            Edge {
                                from: 0,
                                to: 2,
                                cost: 2,
                            },
                            Edge {
                                from: 0,
                                to: 4,
                                cost: 1,
                            },
                        ]
                        .into_iter(),
                    ),
                    1 => Box::new(
                        [
                            Edge {
                                from: 1,
                                to: 0,
                                cost: 0,
                            },
                            Edge {
                                from: 1,
                                to: 2,
                                cost: 3,
                            },
                        ]
                        .into_iter(),
                    ),
                    2 => Box::new(
                        [
                            Edge {
                                from: 2,
                                to: 0,
                                cost: 2,
                            },
                            Edge {
                                from: 2,
                                to: 1,
                                cost: 3,
                            },
                            Edge {
                                from: 2,
                                to: 3,
                                cost: 1,
                            },
                        ]
                        .into_iter(),
                    ),
                    3 => Box::new(
                        [Edge {
                            from: 3,
                            to: 2,
                            cost: 1,
                        }]
                        .into_iter(),
                    ),
                    4 => Box::new(
                        [Edge {
                            from: 4,
                            to: 0,
                            cost: 1,
                        }]
                        .into_iter(),
                    ),
                    _ => Box::new(iter::empty()),
                }
            }
        }

        let network = TestNetwork {};

        // when
        let path = network.find_path(hashset! {0}, hashset! {3}, &|_| 0);

        // then
        assert_eq!(
            path,
            Some(vec![
                Edge {
                    from: 0,
                    to: 2,
                    cost: 2,
                },
                Edge {
                    from: 2,
                    to: 3,
                    cost: 1,
                }
            ])
        );
    }

    #[test]
    fn path_is_not_possible() {
        // given
        //
        // [0] <-1-> [1] [2] <-1-> [3]
        //

        struct TestNetwork {}

        impl Network<usize> for TestNetwork {
            fn edges<'a>(&'a self, from: &'a usize) -> Box<dyn Iterator<Item = Edge<usize>> + 'a> {
                match from {
                    0 => Box::new(
                        [Edge {
                            from: 0,
                            to: 1,
                            cost: 1,
                        }]
                        .into_iter(),
                    ),
                    1 => Box::new(
                        [Edge {
                            from: 1,
                            to: 0,
                            cost: 1,
                        }]
                        .into_iter(),
                    ),
                    2 => Box::new(
                        [Edge {
                            from: 2,
                            to: 3,
                            cost: 1,
                        }]
                        .into_iter(),
                    ),
                    3 => Box::new(
                        [Edge {
                            from: 3,
                            to: 2,
                            cost: 1,
                        }]
                        .into_iter(),
                    ),
                    _ => Box::new(iter::empty()),
                }
            }
        }

        let network = TestNetwork {};

        // when
        let path = network.find_path(hashset! {0}, hashset! {3}, &|_| 0);

        // then
        assert_eq!(path, None);
    }

    #[test]
    fn multiple_from() {
        // given
        //
        // [4] <-1-> [0] <-1-> [1] <-2-> [2] <-1-> [3]
        //            ^___________________^
        //                      3

        struct TestNetwork {}

        impl Network<usize> for TestNetwork {
            fn edges<'a>(&'a self, from: &'a usize) -> Box<dyn Iterator<Item = Edge<usize>> + 'a> {
                match from {
                    0 => Box::new(
                        [
                            Edge {
                                from: 0,
                                to: 1,
                                cost: 1,
                            },
                            Edge {
                                from: 0,
                                to: 2,
                                cost: 3,
                            },
                            Edge {
                                from: 0,
                                to: 4,
                                cost: 1,
                            },
                        ]
                        .into_iter(),
                    ),
                    1 => Box::new(
                        [
                            Edge {
                                from: 1,
                                to: 0,
                                cost: 1,
                            },
                            Edge {
                                from: 1,
                                to: 2,
                                cost: 2,
                            },
                        ]
                        .into_iter(),
                    ),
                    2 => Box::new(
                        [
                            Edge {
                                from: 2,
                                to: 0,
                                cost: 3,
                            },
                            Edge {
                                from: 2,
                                to: 1,
                                cost: 2,
                            },
                            Edge {
                                from: 2,
                                to: 3,
                                cost: 1,
                            },
                        ]
                        .into_iter(),
                    ),
                    3 => Box::new(
                        [Edge {
                            from: 3,
                            to: 2,
                            cost: 1,
                        }]
                        .into_iter(),
                    ),
                    4 => Box::new(
                        [Edge {
                            from: 4,
                            to: 0,
                            cost: 1,
                        }]
                        .into_iter(),
                    ),
                    _ => Box::new(iter::empty()),
                }
            }
        }

        let network = TestNetwork {};

        // when
        let path = network.find_path(hashset! {0, 1}, hashset! {3}, &|_| 0);

        // then
        assert_eq!(
            path,
            Some(vec![
                Edge {
                    from: 1,
                    to: 2,
                    cost: 2,
                },
                Edge {
                    from: 2,
                    to: 3,
                    cost: 1,
                }
            ])
        );
    }

    #[test]
    fn multiple_to() {
        // given
        //
        // [4] <-1-> [0] <-3-> [1] <-1-> [2] <-2-> [3]
        //            ^___________________^
        //                      1

        struct TestNetwork {}

        impl Network<usize> for TestNetwork {
            fn edges<'a>(&'a self, from: &'a usize) -> Box<dyn Iterator<Item = Edge<usize>> + 'a> {
                match from {
                    0 => Box::new(
                        [
                            Edge {
                                from: 0,
                                to: 1,
                                cost: 3,
                            },
                            Edge {
                                from: 0,
                                to: 2,
                                cost: 1,
                            },
                            Edge {
                                from: 0,
                                to: 4,
                                cost: 1,
                            },
                        ]
                        .into_iter(),
                    ),
                    1 => Box::new(
                        [
                            Edge {
                                from: 1,
                                to: 0,
                                cost: 3,
                            },
                            Edge {
                                from: 1,
                                to: 2,
                                cost: 1,
                            },
                        ]
                        .into_iter(),
                    ),
                    2 => Box::new(
                        [
                            Edge {
                                from: 2,
                                to: 0,
                                cost: 1,
                            },
                            Edge {
                                from: 2,
                                to: 1,
                                cost: 1,
                            },
                            Edge {
                                from: 2,
                                to: 3,
                                cost: 2,
                            },
                        ]
                        .into_iter(),
                    ),
                    3 => Box::new(
                        [Edge {
                            from: 3,
                            to: 2,
                            cost: 2,
                        }]
                        .into_iter(),
                    ),
                    4 => Box::new(
                        [Edge {
                            from: 4,
                            to: 0,
                            cost: 1,
                        }]
                        .into_iter(),
                    ),
                    _ => Box::new(iter::empty()),
                }
            }
        }

        let network = TestNetwork {};

        // when
        let path = network.find_path(hashset! {0}, hashset! {1, 3}, &|_| 0);

        // then
        assert_eq!(
            path,
            Some(vec![
                Edge {
                    from: 0,
                    to: 2,
                    cost: 1,
                },
                Edge {
                    from: 2,
                    to: 1,
                    cost: 1,
                }
            ])
        );
    }

    #[test]
    fn multiple_from_and_to() {
        // given
        //
        // [0] & [1] both have edges to [2] & [3]

        struct TestNetwork {}

        impl Network<usize> for TestNetwork {
            fn edges<'a>(&'a self, from: &'a usize) -> Box<dyn Iterator<Item = Edge<usize>> + 'a> {
                match from {
                    0 => Box::new(
                        [
                            Edge {
                                from: 0,
                                to: 2,
                                cost: 3,
                            },
                            Edge {
                                from: 0,
                                to: 3,
                                cost: 2,
                            },
                        ]
                        .into_iter(),
                    ),
                    1 => Box::new(
                        [
                            Edge {
                                from: 1,
                                to: 2,
                                cost: 1, // lowest
                            },
                            Edge {
                                from: 1,
                                to: 3,
                                cost: 4,
                            },
                        ]
                        .into_iter(),
                    ),
                    _ => Box::new(iter::empty()),
                }
            }
        }

        let network = TestNetwork {};

        // when
        let path = network.find_path(hashset! {0, 1}, hashset! {2, 3}, &|_| 0);

        // then
        assert_eq!(
            path,
            Some(vec![Edge {
                from: 1,
                to: 2,
                cost: 1,
            }])
        );
    }

    #[test]
    fn from_equals_to() {
        // given
        //
        // [0] --1-> [1]
        //

        struct TestNetwork {}

        impl Network<usize> for TestNetwork {
            fn edges<'a>(&'a self, from: &'a usize) -> Box<dyn Iterator<Item = Edge<usize>> + 'a> {
                match from {
                    0 => Box::new(
                        [Edge {
                            from: 0,
                            to: 1,
                            cost: 1,
                        }]
                        .into_iter(),
                    ),
                    _ => Box::new(iter::empty()),
                }
            }
        }

        let network = TestNetwork {};

        // when
        let path = network.find_path(hashset! {0}, hashset! {0}, &|_| 0);

        // then
        assert_eq!(path, Some(vec![]));
    }

    #[test]
    fn no_edges() {
        struct TestNetwork {}

        impl Network<usize> for TestNetwork {
            fn edges<'a>(&'a self, _: &'a usize) -> Box<dyn Iterator<Item = Edge<usize>> + 'a> {
                Box::new(iter::empty())
            }
        }

        let network = TestNetwork {};

        // when
        let path = network.find_path(hashset! {0}, hashset! {1}, &|_| 0);

        // then
        assert_eq!(path, None);
    }

    #[test]
    fn empty_from() {
        // given
        //
        // [0] --1-> [1]
        //

        struct TestNetwork {}

        impl Network<usize> for TestNetwork {
            fn edges<'a>(&'a self, from: &'a usize) -> Box<dyn Iterator<Item = Edge<usize>> + 'a> {
                match from {
                    0 => Box::new(
                        [Edge {
                            from: 0,
                            to: 1,
                            cost: 1,
                        }]
                        .into_iter(),
                    ),
                    _ => Box::new(iter::empty()),
                }
            }
        }

        let network = TestNetwork {};

        // when
        let path = network.find_path(hashset! {}, hashset! {1}, &|_| 0);

        // then
        assert_eq!(path, None);
    }

    #[test]
    fn empty_to() {
        // given
        //
        // [0] --1-> [1]
        //

        struct TestNetwork {}

        impl Network<usize> for TestNetwork {
            fn edges<'a>(&'a self, from: &'a usize) -> Box<dyn Iterator<Item = Edge<usize>> + 'a> {
                match from {
                    0 => Box::new(
                        [Edge {
                            from: 0,
                            to: 1,
                            cost: 1,
                        }]
                        .into_iter(),
                    ),
                    _ => Box::new(iter::empty()),
                }
            }
        }

        let network = TestNetwork {};

        // when
        let path = network.find_path(hashset! {0}, hashset! {}, &|_| 0);

        // then
        assert_eq!(path, None);
    }
}