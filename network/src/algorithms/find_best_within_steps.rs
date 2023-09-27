use core::hash::Hash;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::Debug;

use commons::scale::Scale;
use rand::Rng;

use crate::algorithms::get_path;
use crate::model::{Edge, OutNetwork};

struct Node<T> {
    location: T,
    entrance: Option<Edge<T>>,
    steps_from_start: u64,
    cost_from_start: u64,
    score: Option<f32>,
}

impl<T> Ord for Node<T>
where
    T: Eq,
{
    fn cmp(&self, other: &Node<T>) -> Ordering {
        self.steps_from_start.cmp(&other.steps_from_start).reverse()
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

impl <T> Eq for Node<T> where T: Eq {}

pub trait FindBestWithinSteps<T, N> {
    fn find_best_within_steps(
        &self,
        from: HashSet<T>,
        scorer: &dyn Fn(&N, &T) -> Option<f32>,
        can_visit: &dyn Fn(&T) -> bool,
        max_steps: u64,
    ) -> Option<Vec<Edge<T>>>;
}

impl<T, N> FindBestWithinSteps<T, N> for N
where
    T: Copy + Debug + Eq + Hash,
    N: OutNetwork<T>,
{
    fn find_best_within_steps(
        &self,
        from: HashSet<T>,
        scorer: &dyn Fn(&N, &T) -> Option<f32>,
        is_allowed: &dyn Fn(&T) -> bool,
        max_steps: u64,
    ) -> Option<Vec<Edge<T>>> {
        let mut rng = rand::thread_rng();
        let mut closed = HashSet::new();
        let mut entrances = HashMap::new();
        let mut heap = BinaryHeap::new();


        for from in from.iter() {
            if is_allowed(from) {
                let score = scorer(self, from);
                heap.push(Node {
                    location: *from,
                    entrance: None,
                    steps_from_start: 0,
                    cost_from_start: 0,
                    score,
                });
            }
        }

        struct Best<T> {
            location: T,
            score: f32,
        }

        let mut best: Option<Best<T>> = None;

        while let Some(Node {
            location,
            entrance,
            steps_from_start,
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

            if let Some(score) = score {
                // let score = (start_score - score) / (cost_from_start as f32);
                let score = score;
                let scale = Scale::new((0.0, 1.0), (score, score * 2.0));
                let score = scale.scale(rng.gen::<f32>());
                
                best = match best {
                    Some(current) if score < current.score => Some(Best { location, score }),
                    None => Some(Best { location, score }),
                    _ => best,
                };
            }

            for edge in self.edges_out(&location) {
                let to = edge.to;
                if !is_allowed(&to) || closed.contains(&to) {
                    continue;
                }
                let cost_from_start = cost_from_start + edge.cost as u64;
                let steps_from_start = steps_from_start + 1;
                if steps_from_start <= max_steps {
                    heap.push(Node {
                        location: to,
                        entrance: Some(edge),
                        steps_from_start,
                        cost_from_start,
                        score: scorer(self, &to),
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
        let result = network.find_best_within_steps(hashset! {0}, &|_, i| Some(*i), &|_| true, 2);

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
        let result = network.find_best_within_steps(hashset! {0}, &|_, i| Some(*i), &|_| true, 2);

        // then
        assert_eq!(
            result,
            Some(vec![Edge {
                from: 0,
                to: 4,
                cost: 1,
            }])
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
        let result = network.find_best_within_steps(hashset! {2}, &|_, i| Some(*i), &|_| true, 2);

        // then
        assert_eq!(result, Some(vec![]));
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
            &|_, i| Some(i32::from(*i != 0)),
            &|_| true,
            2,
        );

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
            network.find_best_within_steps(hashset! {0, 2}, &|_, i| Some(*i), &|_| true, 4);

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
            &|_, i| match i {
                3 => Some(1),
                5 => Some(1),
                _ => Some(0),
            },
            &|_| true,
            3,
        );

        // then
        assert_eq!(
            result,
            Some(vec![
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
            ])
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
        let result = network.find_best_within_steps(hashset! {}, &|_, _| Some(0), &|_| true, 4);

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
        let result = network.find_best_within_steps(hashset! {0}, &|_, _| Some(0), &|_| true, 4);

        // then
        assert_eq!(result, Some(vec![])); // path to current location
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
        let result = network.find_best_within_steps(hashset! {0}, &|_, i| Some(*i), &|_| true, 1);

        // then
        assert_eq!(
            result,
            Some(vec![Edge {
                from: 0,
                to: 1,
                cost: 1,
            }])
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
        let result = network.find_best_within_steps(hashset! {0}, &|_, i| Some(*i), &|_| true, 0);

        // then
        assert_eq!(result, Some(vec![])); // path to current location
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
            &|_, i| match i {
                2 => None,
                i => Some(*i),
            },
            &|_| true,
            2,
        );

        // then
        assert_eq!(
            result,
            Some(vec![Edge {
                from: 0,
                to: 1,
                cost: 1,
            }])
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
            &|_, i| match i {
                0 => Some(0),
                2 => Some(2),
                _ => None,
            },
            &|_| true,
            2,
        );

        // then
        assert_eq!(
            result,
            Some(vec![
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
            ])
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
            &|_, i| match i {
                0 => Some(0),
                2 => Some(2),
                _ => None,
            },
            &|i| *i != 1,
            2,
        );

        // then
        assert_eq!(result, Some(vec![]));
    }
}
