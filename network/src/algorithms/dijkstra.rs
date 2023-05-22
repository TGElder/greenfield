use std::collections::{HashMap, HashSet};

use crate::model::InNetwork;

pub trait Dijkstra {
    fn cost_to_target(&self, target: &HashSet<usize>) -> HashMap<usize, usize> {
        todo!();
    }
}

impl<T, N> Dijkstra for N
where
    N: InNetwork<T>,
{
    fn cost_to_target(&self, target: &HashSet<usize>) -> HashMap<usize, usize> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use maplit::{hashmap, hashset};
    use std::iter;

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
                                from: 2,
                                to: 0,
                                cost: 2,
                            },
                            Edge {
                                from: 1,
                                to: 0,
                                cost: 1,
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
        let result = network.cost_to_target(hashset! {0});

        // then
        assert_eq!(
            result,
            hashmap! {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 3,
                4 => 4,
                5 => 5
            }
        );
    }
}
