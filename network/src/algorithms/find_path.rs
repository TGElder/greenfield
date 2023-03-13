use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::model::{Edge, Network};

#[derive(Eq)]
struct Node {
    index: usize,
    entrance: Option<Edge>,
    distance_from_start: u64,
    estimated_distance_via_this_node: u64,
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        self.estimated_distance_via_this_node
            .cmp(&other.estimated_distance_via_this_node)
            .reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.estimated_distance_via_this_node == other.estimated_distance_via_this_node
    }
}

pub trait FindPath {
    fn find_path(
        &self,
        from: usize,
        to: usize,
        heuristic: &dyn Fn(usize) -> u64,
    ) -> Option<Vec<Edge>>;
}

impl<T> FindPath for T
where
    T: Network,
{
    fn find_path(
        &self,
        from: usize,
        to: usize,
        heuristic: &dyn Fn(usize) -> u64,
    ) -> Option<Vec<Edge>> {
        let mut closed = HashSet::new();
        let mut entrances = HashMap::new();
        let mut heap = BinaryHeap::new();

        heap.push(Node {
            index: from,
            entrance: None,
            distance_from_start: 0,
            estimated_distance_via_this_node: heuristic(from),
        });

        while let Some(Node {
            index,
            entrance,
            distance_from_start,
            ..
        }) = heap.pop()
        {
            if closed.contains(&index) {
                continue;
            }
            closed.insert(index);

            if let Some(entrance) = entrance {
                entrances.insert(index, entrance);
            }

            if index == to {
                return Some(get_path(&from, &to, &mut entrances));
            }

            for edge in self.edges(&index) {
                let to = edge.to;
                if closed.contains(&to) {
                    continue;
                }
                heap.push(Node {
                    index: to,
                    entrance: Some(edge),
                    distance_from_start: distance_from_start + edge.cost as u64,
                    estimated_distance_via_this_node: heuristic(index),
                });
            }
        }

        None
    }
}

fn get_path(from: &usize, to: &usize, entrances: &mut HashMap<usize, Edge>) -> Vec<Edge> {
    let mut out = vec![];
    let mut focus = *to;
    while focus != *from {
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

    use crate::model::Network;

    use super::*;

    #[test]
    fn path_is_possible() {
        // given
        //
        // [4]-1-[0]-0-[1]-3-[2]-1-[3]
        //        \___________/
        //              2

        struct TestNetwork {}

        impl Network for TestNetwork {
            fn edges<'a>(&'a self, from: &'a usize) -> Box<dyn Iterator<Item = Edge> + 'a> {
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
        let path = network.find_path(0, 3, &|_| 0);

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
        struct TestNetwork {}

        impl Network for TestNetwork {
            fn edges<'a>(&'a self, from: &'a usize) -> Box<dyn Iterator<Item = Edge> + 'a> {
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
        let path = network.find_path(0, 3, &|_| 0);

        // then
        assert_eq!(path, None);
    }

    #[test]
    fn from_equals_to() {
        struct TestNetwork {}

        impl Network for TestNetwork {
            fn edges<'a>(&'a self, from: &'a usize) -> Box<dyn Iterator<Item = Edge> + 'a> {
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
                    _ => Box::new(iter::empty()),
                }
            }
        }

        let network = TestNetwork {};

        // when
        let path = network.find_path(0, 0, &|_| 0);

        // then
        assert_eq!(path, Some(vec![]));
    }
}
