use core::hash::Hash;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::model::InNetwork;

#[derive(Eq, PartialEq)]
struct Node<T> {
    location: T,
    cost_from_targets: u64,
    steps_from_start: u64,
}

impl<T> Ord for Node<T>
where
    T: Eq,
{
    fn cmp(&self, other: &Node<T>) -> Ordering {
        self.cost_from_targets
            .cmp(&other.cost_from_targets)
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

pub trait CostsToTargets<T> {
    fn costs_to_targets(
        &self,
        targets: &HashSet<T>,
        max_steps: Option<u64>,
        max_cost: Option<u64>,
    ) -> HashMap<T, u64>;
}

impl<T, N> CostsToTargets<T> for N
where
    T: Copy + Eq + Hash,
    N: InNetwork<T>,
{
    fn costs_to_targets(
        &self,
        targets: &HashSet<T>,
        max_steps: Option<u64>,
        max_cost: Option<u64>,
    ) -> HashMap<T, u64> {
        let mut heap = BinaryHeap::new();
        let mut closed = HashSet::new();
        let mut out = HashMap::new();

        for location in targets.iter() {
            heap.push(Node {
                location: *location,
                cost_from_targets: 0,
                steps_from_start: 0,
            });
        }

        while let Some(Node {
            location,
            cost_from_targets,
            steps_from_start,
        }) = heap.pop()
        {
            if closed.contains(&location) {
                continue;
            }
            closed.insert(location);
            out.insert(location, cost_from_targets);

            for edge in self.edges_in(&location) {
                let from = edge.from;

                if closed.contains(&from) {
                    continue;
                }

                if let Some(max_steps) = max_steps {
                    if steps_from_start >= max_steps {
                        continue;
                    }
                }

                if let Some(max_cost) = max_cost {
                    if cost_from_targets >= max_cost {
                        continue;
                    }
                }

                heap.push(Node {
                    location: from,
                    cost_from_targets: cost_from_targets + edge.cost as u64,
                    steps_from_start: steps_from_start + 1,
                });
            }
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use maplit::{hashmap, hashset};
    use std::iter;

    use crate::algorithms::costs_to_targets::CostsToTargets;
    use crate::model::{Edge, InNetwork};

    #[test]
    fn basic_case() {
        // given
        //
        // [2] -2-> [0] <-1-- [1] <-1-- [3] <-1-- [4] <-1- [5]
        //           ^_________3_________/

        struct TestNetwork {}

        impl InNetwork<usize> for TestNetwork {
            fn edges_in<'a>(
                &'a self,
                from: &'a usize,
            ) -> Box<dyn Iterator<Item = Edge<usize>> + 'a> {
                match from {
                    0 => Box::new(
                        [
                            Edge {
                                from: 1,
                                to: 0,
                                cost: 1,
                            },
                            Edge {
                                from: 2,
                                to: 0,
                                cost: 2,
                            },
                            Edge {
                                from: 3,
                                to: 0,
                                cost: 3,
                            },
                        ]
                        .into_iter(),
                    ),
                    1 => Box::new(
                        [Edge {
                            from: 3,
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
                    4 => Box::new(
                        [Edge {
                            from: 5,
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
        let result = network.costs_to_targets(&hashset! {0}, None, None);

        // then
        assert_eq!(
            result,
            hashmap! {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 2,
                4 => 3,
                5 => 4
            }
        );
    }

    #[test]
    fn multiple_targets() {
        // given
        //
        // [0] <-1-- [1] --2-> [2]

        struct TestNetwork {}

        impl InNetwork<usize> for TestNetwork {
            fn edges_in<'a>(
                &'a self,
                from: &'a usize,
            ) -> Box<dyn Iterator<Item = Edge<usize>> + 'a> {
                match from {
                    0 => Box::new(
                        [Edge {
                            from: 1,
                            to: 0,
                            cost: 1,
                        }]
                        .into_iter(),
                    ),
                    2 => Box::new(
                        [Edge {
                            from: 1,
                            to: 2,
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
        let result = network.costs_to_targets(&hashset! {0, 2}, None, None);

        // then
        assert_eq!(
            result,
            hashmap! {
                0 => 0,
                1 => 1,
                2 => 0,
            }
        );
    }

    #[test]
    fn no_target() {
        // given
        //
        // [0] <-1-- [1]

        struct TestNetwork {}

        impl InNetwork<usize> for TestNetwork {
            fn edges_in<'a>(
                &'a self,
                from: &'a usize,
            ) -> Box<dyn Iterator<Item = Edge<usize>> + 'a> {
                match from {
                    0 => Box::new(
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
        let result = network.costs_to_targets(&hashset! {}, None, None);

        // then
        assert_eq!(result, hashmap! {},);
    }

    #[test]
    fn no_edges() {
        // given

        struct TestNetwork {}

        impl InNetwork<usize> for TestNetwork {
            fn edges_in<'a>(&'a self, _: &'a usize) -> Box<dyn Iterator<Item = Edge<usize>> + 'a> {
                Box::new(iter::empty())
            }
        }

        let network = TestNetwork {};

        // when
        let result = network.costs_to_targets(&hashset! {0}, None, None);

        // then
        assert_eq!(
            result,
            hashmap! {
                0 => 0,
            },
        );
    }

    #[test]
    fn max_steps() {
        // given
        //
        // [0] <-1-- [1] <-1-- [2]

        struct TestNetwork {}

        impl InNetwork<usize> for TestNetwork {
            fn edges_in<'a>(
                &'a self,
                from: &'a usize,
            ) -> Box<dyn Iterator<Item = Edge<usize>> + 'a> {
                match from {
                    0 => Box::new(
                        [Edge {
                            from: 1,
                            to: 0,
                            cost: 1,
                        }]
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
                    _ => Box::new(iter::empty()),
                }
            }
        }

        let network = TestNetwork {};

        // when
        let result = network.costs_to_targets(&hashset! {0}, Some(1), None);

        // then
        assert_eq!(
            result,
            hashmap! {
                0 => 0,
                1 => 1,
            }
        );
    }

    #[test]
    fn max_steps_zero() {
        // given
        //
        // [0] <-1-- [1]

        struct TestNetwork {}

        impl InNetwork<usize> for TestNetwork {
            fn edges_in<'a>(
                &'a self,
                from: &'a usize,
            ) -> Box<dyn Iterator<Item = Edge<usize>> + 'a> {
                match from {
                    0 => Box::new(
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
        let result = network.costs_to_targets(&hashset! {0}, Some(0), None);

        // then
        assert_eq!(
            result,
            hashmap! {
                0 => 0,
            }
        );
    }

    #[test]
    fn max_cost() {
        // given
        //
        // [0] <-2-- [1] <-2-- [2]

        struct TestNetwork {}

        impl InNetwork<usize> for TestNetwork {
            fn edges_in<'a>(
                &'a self,
                from: &'a usize,
            ) -> Box<dyn Iterator<Item = Edge<usize>> + 'a> {
                match from {
                    0 => Box::new(
                        [Edge {
                            from: 1,
                            to: 0,
                            cost: 2,
                        }]
                        .into_iter(),
                    ),
                    1 => Box::new(
                        [Edge {
                            from: 2,
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
        let result = network.costs_to_targets(&hashset! {0}, None, Some(2));

        // then
        assert_eq!(
            result,
            hashmap! {
                0 => 0,
                1 => 2,
            }
        );
    }

    #[test]
    fn max_cost_zero() {
        // given
        //
        // [0] <-1-- [1]

        struct TestNetwork {}

        impl InNetwork<usize> for TestNetwork {
            fn edges_in<'a>(
                &'a self,
                from: &'a usize,
            ) -> Box<dyn Iterator<Item = Edge<usize>> + 'a> {
                match from {
                    0 => Box::new(
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
        let result = network.costs_to_targets(&hashset! {0}, None, Some(0));

        // then
        assert_eq!(
            result,
            hashmap! {
                0 => 0,
            }
        );
    }
}
