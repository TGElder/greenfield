use core::hash::Hash;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::Debug;

use crate::algorithms::get_path;
use crate::model::{Edge, OutNetwork};

struct Node<S, T> {
    location: T,
    entrance: Option<Edge<T>>,
    steps_from_start: u64,
    score: Option<S>,
}

impl<S, T> Ord for Node<S, T>
where
    T: Eq,
{
    fn cmp(&self, other: &Node<S, T>) -> Ordering {
        self.steps_from_start.cmp(&other.steps_from_start).reverse()
    }
}

impl<S, T> PartialOrd for Node<S, T>
where
    T: Eq,
{
    fn partial_cmp(&self, other: &Node<S, T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<S, T> PartialEq for Node<S, T>
where
    T: Eq,
{
    fn eq(&self, other: &Node<S, T>) -> bool {
        self.location == other.location && self.entrance == other.entrance
    }
}

impl<S, T> Eq for Node<S, T> where T: Eq {}

#[derive(Debug, PartialEq)]
pub struct Result<S, T> {
    pub score: S,
    pub path: Vec<Edge<T>>,
}

pub trait FindBestWithinSteps<S, T, N> {
    fn find_best_within_steps(
        &self,
        from: HashSet<T>,
        scorer: &mut dyn FnMut(&N, &T) -> Option<S>,
        can_visit: &mut dyn FnMut(&T) -> bool,
        max_steps: u64,
    ) -> Option<Result<S, T>>;
}

impl<S, T, N> FindBestWithinSteps<S, T, N> for N
where
    S: Ord,
    T: Copy + Debug + Eq + Hash,
    N: OutNetwork<T>,
{
    fn find_best_within_steps(
        &self,
        from: HashSet<T>,
        scorer: &mut dyn FnMut(&N, &T) -> Option<S>,
        is_allowed: &mut dyn FnMut(&T) -> bool,
        max_steps: u64,
    ) -> Option<Result<S, T>> {
        let mut closed = HashSet::new();
        let mut entrances = HashMap::new();
        let mut heap = BinaryHeap::new();

        for from in from.iter() {
            if is_allowed(from) {
                heap.push(Node {
                    location: *from,
                    entrance: None,
                    steps_from_start: 0,
                    score: scorer(self, from),
                });
            }
        }

        struct Best<S, T> {
            score: S,
            location: T,
        }

        let mut best: Option<Best<S, T>> = None;

        while let Some(Node {
            location,
            entrance,
            steps_from_start,
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

            if let Some(score) = score {
                best = match best {
                    Some(current) if score > current.score => Some(Best { location, score }),
                    None => Some(Best { location, score }),
                    _ => best,
                };
            }

            for edge in self.edges_out(&location) {
                let to = edge.to;
                if !is_allowed(&to) || closed.contains(&to) {
                    continue;
                }
                let steps_from_start = steps_from_start + 1;
                if steps_from_start <= max_steps {
                    heap.push(Node {
                        location: to,
                        entrance: Some(edge),
                        steps_from_start,
                        score: scorer(self, &to),
                    });
                }
            }
        }

        best.map(|Best { location, score }| Result {
            score,
            path: get_path(&from, &location, &mut entrances),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::iter;

    use maplit::hashset;

    use super::*;

    #[test]
    fn best_is_at_max_steps() {
        // given
        // [2] <- [1] <- [0] -> [3] -> [4]

        struct TestNetwork {}

        impl OutNetwork<usize> for TestNetwork {
            fn edges_out<'a>(
                &'a self,
                from: &'a usize,
            ) -> Box<dyn Iterator<Item = Edge<usize>> + 'a> {
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
                                cost: 1,
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
        let result =
            network.find_best_within_steps(hashset! {0}, &mut |_, i| Some(*i), &mut |_| true, 2);

        // then
        assert_eq!(
            result,
            Some(Result {
                score: 4,
                path: vec![
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
                ]
            })
        );
    }

    #[test]
    fn best_is_not_at_max_steps() {
        // given
        //
        // [1] <- [2] <- [0] -> [4] -> [3]

        struct TestNetwork {}

        impl OutNetwork<usize> for TestNetwork {
            fn edges_out<'a>(
                &'a self,
                from: &'a usize,
            ) -> Box<dyn Iterator<Item = Edge<usize>> + 'a> {
                match from {
                    0 => Box::new(
                        [
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
                        [Edge {
                            from: 2,
                            to: 1,
                            cost: 1,
                        }]
                        .into_iter(),
                    ),
                    3 => Box::new(
                        [Edge {
                            from: 4,
                            to: 3,
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
        let result =
            network.find_best_within_steps(hashset! {0}, &mut |_, i| Some(*i), &mut |_| true, 2);

        // then
        assert_eq!(
            result,
            Some(Result {
                score: 4,
                path: vec![Edge {
                    from: 0,
                    to: 4,
                    cost: 1,
                }]
            }),
        );
    }

    #[test]
    fn best_is_starting_location() {
        // given
        //
        // [1] <- [2] -> [0]

        struct TestNetwork {}

        impl OutNetwork<usize> for TestNetwork {
            fn edges_out<'a>(
                &'a self,
                from: &'a usize,
            ) -> Box<dyn Iterator<Item = Edge<usize>> + 'a> {
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
        let result =
            network.find_best_within_steps(hashset! {2}, &mut |_, i| Some(*i), &mut |_| true, 2);

        // then
        assert_eq!(
            result,
            Some(Result {
                score: 2,
                path: vec![]
            })
        );
    }

    #[test]
    fn tied_best() {
        // given
        //
        // [1] <- [0] -> [2]

        struct TestNetwork {}

        impl OutNetwork<usize> for TestNetwork {
            fn edges_out<'a>(
                &'a self,
                from: &'a usize,
            ) -> Box<dyn Iterator<Item = Edge<usize>> + 'a> {
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
        let result = network.find_best_within_steps(
            hashset! {0},
            &mut |_, i| Some(i32::from(*i != 0)),
            &mut |_| true,
            2,
        );

        // then
        assert!(
            result
                == Some(Result {
                    score: 1,
                    path: vec![Edge {
                        from: 0,
                        to: 1,
                        cost: 1,
                    }]
                })
                || result
                    == Some(Result {
                        score: 1,
                        path: vec![Edge {
                            from: 0,
                            to: 2,
                            cost: 1,
                        }]
                    })
        );
    }

    #[test]
    fn multiple_from() {
        // given
        //
        // [1] <- [0] <-> [2] -> [3]

        struct TestNetwork {}

        impl OutNetwork<usize> for TestNetwork {
            fn edges_out<'a>(
                &'a self,
                from: &'a usize,
            ) -> Box<dyn Iterator<Item = Edge<usize>> + 'a> {
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
        let result =
            network.find_best_within_steps(hashset! {0, 2}, &mut |_, i| Some(*i), &mut |_| true, 4);

        // then
        assert_eq!(
            result,
            Some(Result {
                score: 3,
                path: vec![Edge {
                    from: 2,
                    to: 3,
                    cost: 1,
                }]
            })
        );
    }

    #[test]
    fn ties_are_broken_by_steps() {
        // given
        //
        // [3] <- [2] <- [1] <- [0] -> [4] -> [5]

        struct TestNetwork {}

        impl OutNetwork<usize> for TestNetwork {
            fn edges_out<'a>(
                &'a self,
                from: &'a usize,
            ) -> Box<dyn Iterator<Item = Edge<usize>> + 'a> {
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
                                to: 4,
                                cost: 1,
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
                    2 => Box::new(
                        [Edge {
                            from: 2,
                            to: 3,
                            cost: 1,
                        }]
                        .into_iter(),
                    ),
                    4 => Box::new(
                        [Edge {
                            from: 4,
                            to: 5,
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
        let result = network.find_best_within_steps(
            hashset! {0},
            &mut |_, i| match i {
                3 => Some(1),
                5 => Some(1),
                _ => Some(0),
            },
            &mut |_| true,
            3,
        );

        // then
        assert_eq!(
            result,
            Some(Result {
                score: 1,
                path: vec![
                    Edge {
                        from: 0,
                        to: 4,
                        cost: 1,
                    },
                    Edge {
                        from: 4,
                        to: 5,
                        cost: 1,
                    }
                ]
            })
        );
    }

    #[test]
    fn empty_from() {
        // given
        //
        // [0] -> [1]

        struct TestNetwork {}

        impl OutNetwork<usize> for TestNetwork {
            fn edges_out<'a>(
                &'a self,
                from: &'a usize,
            ) -> Box<dyn Iterator<Item = Edge<usize>> + 'a> {
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
        let result =
            network.find_best_within_steps(hashset! {}, &mut |_, _| Some(0), &mut |_| true, 4);

        // then
        assert_eq!(result, None);
    }

    #[test]
    fn no_edges() {
        // given

        struct TestNetwork {}

        impl OutNetwork<usize> for TestNetwork {
            fn edges_out<'a>(&'a self, _: &'a usize) -> Box<dyn Iterator<Item = Edge<usize>> + 'a> {
                Box::new(iter::empty())
            }
        }

        let network = TestNetwork {};

        // when
        let result =
            network.find_best_within_steps(hashset! {0}, &mut |_, _| Some(0), &mut |_| true, 4);

        // then
        assert_eq!(
            result,
            Some(Result {
                score: 0,
                path: vec![]
            })
        ); // path to current location
    }

    #[test]
    fn best_node_not_within_steps() {
        // given
        //
        // [0] -> [1] -> [2]

        struct TestNetwork {}

        impl OutNetwork<usize> for TestNetwork {
            fn edges_out<'a>(
                &'a self,
                from: &'a usize,
            ) -> Box<dyn Iterator<Item = Edge<usize>> + 'a> {
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
        let result =
            network.find_best_within_steps(hashset! {0}, &mut |_, i| Some(*i), &mut |_| true, 1);

        // then
        assert_eq!(
            result,
            Some(Result {
                score: 1,
                path: vec![Edge {
                    from: 0,
                    to: 1,
                    cost: 1,
                }]
            })
        );
    }

    #[test]
    fn max_steps_zero() {
        // given
        //
        // [0] -> [1]

        struct TestNetwork {}

        impl OutNetwork<usize> for TestNetwork {
            fn edges_out<'a>(
                &'a self,
                from: &'a usize,
            ) -> Box<dyn Iterator<Item = Edge<usize>> + 'a> {
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
        let result =
            network.find_best_within_steps(hashset! {0}, &mut |_, i| Some(*i), &mut |_| true, 0);

        // then
        assert_eq!(
            result,
            Some(Result {
                score: 0,
                path: vec![]
            })
        ); // path to current location
    }

    #[test]
    fn must_not_finish_on_no_score_node() {
        // given
        //
        // [1] <- [0] -> [2]

        struct TestNetwork {}

        impl OutNetwork<usize> for TestNetwork {
            fn edges_out<'a>(
                &'a self,
                from: &'a usize,
            ) -> Box<dyn Iterator<Item = Edge<usize>> + 'a> {
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
        let result = network.find_best_within_steps(
            hashset! {0},
            &mut |_, i| match i {
                2 => None,
                i => Some(*i),
            },
            &mut |_| true,
            2,
        );

        // then
        assert_eq!(
            result,
            Some(Result {
                score: 1,
                path: vec![Edge {
                    from: 0,
                    to: 1,
                    cost: 1,
                }]
            })
        );
    }

    #[test]
    fn may_pass_through_can_visit_node() {
        // given
        //
        // [0] -> [1] -> [2]

        struct TestNetwork {}

        impl OutNetwork<usize> for TestNetwork {
            fn edges_out<'a>(
                &'a self,
                from: &'a usize,
            ) -> Box<dyn Iterator<Item = Edge<usize>> + 'a> {
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
        let result = network.find_best_within_steps(
            hashset! {0},
            &mut |_, i| match i {
                0 => Some(0),
                2 => Some(2),
                _ => None,
            },
            &mut |_| true,
            2,
        );

        // then
        assert_eq!(
            result,
            Some(Result {
                score: 2,
                path: vec![
                    Edge {
                        from: 0,
                        to: 1,
                        cost: 1,
                    },
                    Edge {
                        from: 1,
                        to: 2,
                        cost: 1,
                    }
                ]
            })
        );
    }

    #[test]
    fn may_not_pass_through_cannot_visit_node() {
        // given
        //
        // [0] -> [1] -> [2]

        struct TestNetwork {}

        impl OutNetwork<usize> for TestNetwork {
            fn edges_out<'a>(
                &'a self,
                from: &'a usize,
            ) -> Box<dyn Iterator<Item = Edge<usize>> + 'a> {
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
        let result = network.find_best_within_steps(
            hashset! {0},
            &mut |_, i| match i {
                0 => Some(0),
                2 => Some(2),
                _ => None,
            },
            &mut |i| *i != 1,
            2,
        );

        // then
        assert_eq!(
            result,
            Some(Result {
                score: 0,
                path: vec![]
            })
        );
    }
}
