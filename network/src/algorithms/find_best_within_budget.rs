use core::hash::Hash;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::algorithms::get_path;
use crate::model::{Edge, Network};

#[derive(Eq)]
struct Node<S, T> {
    location: T,
    entrance: Option<Edge<T>>,
    cost_from_start: u64,
    score: S,
}

impl<S, T> Ord for Node<S, T>
where
    S: Ord + Eq,
    T: Eq,
{
    fn cmp(&self, other: &Node<S, T>) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl<S, T> PartialOrd for Node<S, T>
where
    S: Ord + Eq,
    T: Eq,
{
    fn partial_cmp(&self, other: &Node<S, T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<S, T> PartialEq for Node<S, T>
where
    S: Eq,
    T: Eq,
{
    fn eq(&self, other: &Node<S, T>) -> bool {
        self.location == other.location && self.entrance == other.entrance
    }
}

pub trait FindBestWithinBudget<S, T> {
    fn find_best_within_budget(
        &self,
        from: HashSet<T>,
        scorer: &dyn Fn(&T) -> S,
        max_cost: u64,
    ) -> Option<Vec<Edge<T>>>;
}

impl<S, T, N> FindBestWithinBudget<S, T> for N
where
    S: Copy + Eq + Hash + Ord,
    T: Copy + Eq + Hash,
    N: Network<T>,
{
    fn find_best_within_budget(
        &self,
        from: HashSet<T>,
        scorer: &dyn Fn(&T) -> S,
        max_cost: u64,
    ) -> Option<Vec<Edge<T>>> {
        let mut closed = HashSet::new();
        let mut entrances = HashMap::new();
        let mut heap = BinaryHeap::new();

        for from in from.iter() {
            heap.push(Node {
                location: *from,
                entrance: None,
                cost_from_start: 0,
                score: scorer(from),
            });
        }

        struct Best<S, T> {
            location: T,
            score: S,
        }

        let mut best: Option<Best<S, T>> = None;

        while let Some(Node {
            location,
            entrance,
            cost_from_start,
            score,
        }) = heap.pop()
        {
            if closed.contains(&location) {
                continue;
            }
            closed.insert(location);

            if let Some(entrance) = entrance {
                entrances.insert(location, entrance);
            }

            best = match best {
                Some(current) if score > current.score => Some(Best { location, score }),
                None => Some(Best { location, score }),
                _ => best,
            };

            for edge in self.edges(&location) {
                let to = edge.to;
                if closed.contains(&to) {
                    continue;
                }
                let cost_from_start = cost_from_start + edge.cost as u64;
                if cost_from_start <= max_cost {
                    heap.push(Node {
                        location: to,
                        entrance: Some(edge),
                        cost_from_start,
                        score: scorer(&to),
                    });
                }
            }
        }

        best.map(|Best { location, .. }| get_path(&from, &location, &mut entrances))
    }
}

#[cfg(test)]
mod tests {
    use std::iter;

    use maplit::hashset;

    use super::*;

    #[test]
    fn best_is_furthest_within_budget() {
        // given
        // [2] <-1- [1] <-1- [0] -2-> [3] -2-> [4]

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
                                to: 3,
                                cost: 2,
                            },
                        ]
                        .into_iter(),
                    ),
                    1 => Box::new(
                        [Edge {
                            from: 1,
                            to: 2,
                            cost: 1,
                        }]
                        .into_iter(),
                    ),
                    3 => Box::new(
                        [Edge {
                            from: 3,
                            to: 4,
                            cost: 2,
                        }]
                        .into_iter(),
                    ),
                    _ => Box::new(iter::empty()),
                }
            }
        }

        let network = TestNetwork {};

        // when
        let result = network.find_best_within_budget(hashset! {0}, &|i| *i, 4);

        // then
        assert_eq!(
            result,
            Some(vec![
                Edge {
                    from: 0,
                    to: 3,
                    cost: 2,
                },
                Edge {
                    from: 3,
                    to: 4,
                    cost: 2,
                }
            ])
        );
    }

    #[test]
    fn best_is_not_furthest_within_budget() {
        // given
        // [2] <-2- [1] <-2- [0] -1-> [3] -1-> [4]

        struct TestNetwork {}

        impl Network<usize> for TestNetwork {
            fn edges<'a>(&'a self, from: &'a usize) -> Box<dyn Iterator<Item = Edge<usize>> + 'a> {
                match from {
                    0 => Box::new(
                        [
                            Edge {
                                from: 0,
                                to: 1,
                                cost: 2,
                            },
                            Edge {
                                from: 0,
                                to: 3,
                                cost: 1,
                            },
                        ]
                        .into_iter(),
                    ),
                    1 => Box::new(
                        [Edge {
                            from: 1,
                            to: 2,
                            cost: 2,
                        }]
                        .into_iter(),
                    ),
                    3 => Box::new(
                        [Edge {
                            from: 3,
                            to: 4,
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
        let result = network.find_best_within_budget(hashset! {0}, &|i| *i, 4);

        // then
        assert_eq!(
            result,
            Some(vec![
                Edge {
                    from: 0,
                    to: 3,
                    cost: 1,
                },
                Edge {
                    from: 3,
                    to: 4,
                    cost: 1,
                }
            ])
        );
    }

    #[test]
    fn best_is_starting_location() {
        // given
        // [1] <-1- [2] -1-> [0]

        struct TestNetwork {}

        impl Network<usize> for TestNetwork {
            fn edges<'a>(&'a self, from: &'a usize) -> Box<dyn Iterator<Item = Edge<usize>> + 'a> {
                match from {
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
                        ]
                        .into_iter(),
                    ),
                    _ => Box::new(iter::empty()),
                }
            }
        }

        let network = TestNetwork {};

        // when
        let result = network.find_best_within_budget(hashset! {2}, &|i| *i, 4);

        // then
        assert_eq!(result, Some(vec![]));
    }

    #[test]
    fn tied_best() {
        // given
        // [1] <-1- [0] -1-> [2]

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
                                cost: 1,
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
        let result = network.find_best_within_budget(hashset! {0}, &|i| i32::from(*i != 0), 4);

        // then
        assert!(
            result
                == Some(vec![Edge {
                    from: 0,
                    to: 1,
                    cost: 1,
                }])
                || result
                    == Some(vec![Edge {
                        from: 0,
                        to: 2,
                        cost: 1,
                    }])
        );
    }

    #[test]
    fn multiple_from() {
        // given
        // [1] <-1- [0] <-2-> [2] -1-> [3]

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
                                cost: 2,
                            },
                        ]
                        .into_iter(),
                    ),
                    1 => Box::new(
                        [
                            Edge {
                                from: 2,
                                to: 0,
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
                    _ => Box::new(iter::empty()),
                }
            }
        }

        let network = TestNetwork {};

        // when
        let result = network.find_best_within_budget(hashset! {0, 2}, &|i| *i, 4);

        // then
        assert_eq!(
            result,
            Some(vec![Edge {
                from: 2,
                to: 3,
                cost: 1,
            }])
        );
    }

    #[test]
    fn empty_from() {
        // given
        //
        // [0] -1-> [1]
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
        let path = network.find_best_within_budget(hashset! {}, &|_| 0, 4);

        // then
        assert_eq!(path, None);
    }

    #[test]
    fn no_edges() {
        // given

        struct TestNetwork {}

        impl Network<usize> for TestNetwork {
            fn edges<'a>(&'a self, _: &'a usize) -> Box<dyn Iterator<Item = Edge<usize>> + 'a> {
                Box::new(iter::empty())
            }
        }

        let network = TestNetwork {};

        // when
        let path = network.find_best_within_budget(hashset! {0}, &|_| 0, 4);

        // then
        assert_eq!(path, Some(vec![])); // path to current location
    }

    #[test]
    fn best_node_not_within_budget() {
        // given
        //
        // [0] -2-> [1]
        //

        struct TestNetwork {}

        impl Network<usize> for TestNetwork {
            fn edges<'a>(&'a self, from: &'a usize) -> Box<dyn Iterator<Item = Edge<usize>> + 'a> {
                match from {
                    0 => Box::new(
                        [Edge {
                            from: 0,
                            to: 1,
                            cost: 2,
                        }]
                        .into_iter(),
                    ),
                    _ => Box::new(iter::empty()),
                }
            }
        }

        let network = TestNetwork {};

        // when
        let path = network.find_best_within_budget(hashset! {0}, &|i| *i, 1);

        // then
        assert_eq!(path, Some(vec![])); // path to current location
    }

    #[test]
    fn max_cost_zero() {
        // given
        //
        // [0] -1-> [1]
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
        let path = network.find_best_within_budget(hashset! {0}, &|i| *i, 0);

        // then
        assert_eq!(path, Some(vec![])); // path to current location
    }
}