use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

use crate::model::Edge;

#[derive(Debug, PartialEq)]
pub struct Result<T> {
    pub cost: u64,
    pub path: Vec<T>,
}

pub fn floyd_warshall<T>(edges: &[Edge<T>]) -> HashMap<(T, T), Result<T>>
where
    T: Copy + Eq + Hash,
{
    let mut nodes = edges
        .iter()
        .flat_map(|edge| [edge.from, edge.to])
        .collect::<HashSet<_>>();
    let nodes = nodes.drain().collect::<Vec<_>>();

    let node_count = nodes.len();
    let node_to_index = nodes
        .iter()
        .enumerate()
        .map(|(i, node)| (node, i))
        .collect::<HashMap<_, _>>();

    let edges = edges.iter().map(|edge| Edge {
        from: node_to_index[&edge.from],
        to: node_to_index[&edge.to],
        cost: edge.cost,
    });

    let result = internal(&node_count, edges);

    let mut out = HashMap::new();
    for i in 0..node_count {
        for j in 0..node_count {
            if let Some(data) = &result[i][j] {
                out.insert(
                    (nodes[i], nodes[j]),
                    Result {
                        cost: data.cost,
                        path: path(&i, &j, &result).drain(..).map(|i| nodes[i]).collect(),
                    },
                );
            }
        }
    }

    out
}

#[derive(Clone, Debug, Default, PartialEq)]
struct Data {
    cost: u64,
    penultimate: usize,
}

fn internal(
    &node_count: &usize,
    edges: impl Iterator<Item = Edge<usize>>,
) -> Vec<Vec<Option<Data>>> {
    let mut out = vec![vec![None; node_count]; node_count];

    (0..node_count).for_each(|i| {
        out[i][i] = Some(Data {
            cost: 0,
            penultimate: i,
        })
    });

    for edge in edges {
        out[edge.from][edge.to] = Some(Data {
            cost: edge.cost as u64,
            penultimate: edge.from,
        })
    }

    for k in 0..node_count {
        for i in 0..node_count {
            for j in 0..node_count {
                let Some(a) = &out[i][k] else {
                    continue;
                };
                let Some(b) = &out[k][j] else {
                    continue;
                };

                if match out[i][j] {
                    Some(Data { cost, .. }) => a.cost + b.cost < cost,
                    None => true,
                } {
                    out[i][j] = Some(Data {
                        cost: a.cost + b.cost,
                        penultimate: b.penultimate,
                    })
                }
            }
        }
    }

    out
}

fn path(&from: &usize, &to: &usize, data: &[Vec<Option<Data>>]) -> Vec<usize> {
    let mut out = vec![to];

    let mut focus = to;

    while focus != from {
        focus = data[from][focus].as_ref().unwrap().penultimate;
        out.push(focus);
    }

    out.reverse();

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // given
        // [0] <-1-> [1] <-2-> [2] <-3-> [3]       [4] --4-> [5]

        let edges = [
            Edge {
                from: 0,
                to: 1,
                cost: 1,
            },
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
            Edge {
                from: 2,
                to: 1,
                cost: 2,
            },
            Edge {
                from: 2,
                to: 3,
                cost: 3,
            },
            Edge {
                from: 3,
                to: 2,
                cost: 3,
            },
            Edge {
                from: 4,
                to: 5,
                cost: 4,
            },
        ];

        // when
        let result = floyd_warshall(&edges);

        // then
        let expected = HashMap::from([
            (
                (0, 0),
                Result {
                    cost: 0,
                    path: vec![0],
                },
            ),
            (
                (0, 1),
                Result {
                    cost: 1,
                    path: vec![0, 1],
                },
            ),
            (
                (0, 2),
                Result {
                    cost: 3,
                    path: vec![0, 1, 2],
                },
            ),
            (
                (0, 3),
                Result {
                    cost: 6,
                    path: vec![0, 1, 2, 3],
                },
            ),
            (
                (1, 0),
                Result {
                    cost: 1,
                    path: vec![1, 0],
                },
            ),
            (
                (1, 1),
                Result {
                    cost: 0,
                    path: vec![1],
                },
            ),
            (
                (1, 2),
                Result {
                    cost: 2,
                    path: vec![1, 2],
                },
            ),
            (
                (1, 3),
                Result {
                    cost: 5,
                    path: vec![1, 2, 3],
                },
            ),
            (
                (2, 0),
                Result {
                    cost: 3,
                    path: vec![2, 1, 0],
                },
            ),
            (
                (2, 1),
                Result {
                    cost: 2,
                    path: vec![2, 1],
                },
            ),
            (
                (2, 2),
                Result {
                    cost: 0,
                    path: vec![2],
                },
            ),
            (
                (2, 3),
                Result {
                    cost: 3,
                    path: vec![2, 3],
                },
            ),
            (
                (3, 0),
                Result {
                    cost: 6,
                    path: vec![3, 2, 1, 0],
                },
            ),
            (
                (3, 1),
                Result {
                    cost: 5,
                    path: vec![3, 2, 1],
                },
            ),
            (
                (3, 2),
                Result {
                    cost: 3,
                    path: vec![3, 2],
                },
            ),
            (
                (3, 3),
                Result {
                    cost: 0,
                    path: vec![3],
                },
            ),
            (
                (4, 4),
                Result {
                    cost: 0,
                    path: vec![4],
                },
            ),
            (
                (4, 5),
                Result {
                    cost: 4,
                    path: vec![4, 5],
                },
            ),
            (
                (5, 5),
                Result {
                    cost: 0,
                    path: vec![5],
                },
            ),
        ]);
        for i in 0..6 {
            for j in 0..6 {
                let pair = (i, j);
                println!("{:?}", pair);
                assert_eq!(result.get(&pair), expected.get(&pair));
            }
        }
    }
}
