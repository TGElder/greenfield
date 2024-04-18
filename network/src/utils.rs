use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::iter;

use crate::model::{Edge, InNetwork, OutNetwork};

pub struct MaterializedInNetwork<T> {
    pub edges_in: HashMap<T, Vec<Edge<T>>>,
}

impl<T> MaterializedInNetwork<T>
where
    T: Copy + Eq + Hash,
{
    pub fn from_out_network(
        out_network: &dyn OutNetwork<T>,
        nodes: &HashSet<T>,
    ) -> MaterializedInNetwork<T> {
        let mut edges_in = HashMap::with_capacity(nodes.len());

        for node in nodes {
            for edge in out_network.edges_out(node) {
                if nodes.contains(&edge.to) {
                    edges_in.entry(edge.to).or_insert_with(Vec::new).push(edge);
                }
            }
        }

        MaterializedInNetwork { edges_in }
    }
}

impl<T> InNetwork<T> for MaterializedInNetwork<T>
where
    T: Copy + Eq + Hash + PartialEq,
{
    fn edges_in<'a>(&'a self, to: &'a T) -> Box<dyn Iterator<Item = Edge<T>> + 'a> {
        match self.edges_in.get(to) {
            Some(edges) => Box::new(edges.iter().copied()),
            None => Box::new(iter::empty()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn from_out_network() {
        // given
        // [0] --0-> [1] <-1-- [2] <-2-> [3]       [4]

        struct TestOutNetwork {}

        impl OutNetwork<usize> for TestOutNetwork {
            fn edges_out<'a>(
                &'a self,
                from: &'a usize,
            ) -> Box<dyn Iterator<Item = Edge<usize>> + 'a> {
                match from {
                    0 => Box::new(
                        [Edge {
                            from: 0,
                            to: 1,
                            cost: 0,
                        }]
                        .into_iter(),
                    ),
                    2 => Box::new(
                        [
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
                    _ => Box::new(iter::empty()),
                }
            }
        }

        // when
        let in_network = MaterializedInNetwork::from_out_network(
            &TestOutNetwork {},
            &HashSet::from([0, 1, 2, 3, 4]),
        );

        // then
        assert_eq!(in_network.edges_in(&0).count(), 0);
        assert_eq!(
            in_network
                .edges_in(&1)
                .map(|Edge { from, to, cost }| (from, to, cost))
                .collect::<HashSet<_>>(),
            HashSet::from([(0, 1, 0,), (2, 1, 1,)])
        );
        assert_eq!(
            in_network
                .edges_in(&2)
                .map(|Edge { from, to, cost }| (from, to, cost))
                .collect::<HashSet<_>>(),
            HashSet::from([(3, 2, 2)])
        );
        assert_eq!(
            in_network
                .edges_in(&3)
                .map(|Edge { from, to, cost }| (from, to, cost))
                .collect::<HashSet<_>>(),
            HashSet::from([(2, 3, 2)])
        );
        assert_eq!(in_network.edges_in(&4).count(), 0);
    }

    #[test]
    fn from_out_network_should_only_include_listed_nodes() {
        // given
        // [0] --0-> [1]

        struct TestOutNetwork {}

        impl OutNetwork<usize> for TestOutNetwork {
            fn edges_out<'a>(
                &'a self,
                from: &'a usize,
            ) -> Box<dyn Iterator<Item = Edge<usize>> + 'a> {
                match from {
                    0 => Box::new(
                        [Edge {
                            from: 0,
                            to: 1,
                            cost: 0,
                        }]
                        .into_iter(),
                    ),
                    _ => Box::new(iter::empty()),
                }
            }
        }

        // when
        let in_network =
            MaterializedInNetwork::from_out_network(&TestOutNetwork {}, &HashSet::from([0]));

        // then
        assert_eq!(in_network.edges_in(&1).count(), 0);
    }
}
