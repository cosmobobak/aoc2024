use std::fmt::Display;

use anyhow::{bail, Result};
use fxhash::{FxHashMap, FxHashSet};

use graphsearch::{graph::{Graph, WeightedGraph}, graphsearcher::GraphSearcher};

use crate::AocResult;

#[derive(Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    const fn left(self) -> Self {
        match self {
            Self::N => Self::E,
            Self::E => Self::S,
            Self::S => Self::W,
            Self::W => Self::N,
        }
    }
    const fn right(self) -> Self {
        match self {
            Self::N => Self::W,
            Self::E => Self::N,
            Self::S => Self::E,
            Self::W => Self::S,
        }
    }
    const fn offset(self) -> (isize, isize) {
        match self {
            Self::N => (-1, 0),
            Self::E => (0, 1),
            Self::S => (1, 0),
            Self::W => (0, -1),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Node {
    dir: Dir,
    loc: (usize, usize),
}

impl Display for Node {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Edge {
    from: Node,
    to: Node,
}

impl Display for Edge {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

struct Maze {
    inner: FxHashSet<(usize, usize)>,
    root: Node,
    goal: (usize, usize),
}

impl Graph for Maze {
    type Node = Node;
    type Edge = Edge;

    fn root(&self) -> Self::Node {
        self.root
    }

    fn children(&self, node: Self::Node) -> Vec<Self::Node> {
        let offset = node.dir.offset();
        self.inner
            .get(&(
                node.loc.0.wrapping_add_signed(offset.0),
                node.loc.1.wrapping_add_signed(offset.1),
            ))
            .map_or_else(
                || {
                    vec![
                        Node {
                            dir: node.dir.left(),
                            loc: node.loc,
                        },
                        Node {
                            dir: node.dir.right(),
                            loc: node.loc,
                        },
                    ]
                },
                |next| {
                    vec![
                        Node {
                            dir: node.dir,
                            loc: *next,
                        },
                        Node {
                            dir: node.dir.left(),
                            loc: node.loc,
                        },
                        Node {
                            dir: node.dir.right(),
                            loc: node.loc,
                        },
                    ]
                },
            )
    }

    fn edges(&self, node: Self::Node) -> Vec<Self::Edge> {
        let offset = node.dir.offset();
        self.inner
            .get(&(
                node.loc.0.wrapping_add_signed(offset.0),
                node.loc.1.wrapping_add_signed(offset.1),
            ))
            .map_or_else(
                || {
                    vec![
                        Edge {
                            from: node,
                            to: Node {
                                dir: node.dir.left(),
                                loc: node.loc,
                            },
                        },
                        Edge {
                            from: node,
                            to: Node {
                                dir: node.dir.right(),
                                loc: node.loc,
                            },
                        },
                    ]
                },
                |next| {
                    vec![
                        Edge {
                            from: node,
                            to: Node {
                                dir: node.dir,
                                loc: *next,
                            },
                        },
                        Edge {
                            from: node,
                            to: Node {
                                dir: node.dir.left(),
                                loc: node.loc,
                            },
                        },
                        Edge {
                            from: node,
                            to: Node {
                                dir: node.dir.right(),
                                loc: node.loc,
                            },
                        },
                    ]
                },
            )
    }

    fn is_goal(&self, node: Self::Node) -> bool {
        node.loc == self.goal
    }
}

impl WeightedGraph for Maze {
    fn edge_weight(&self, from: Self::Node, to: Self::Node) -> i64 {
        if to.loc == from.loc {
            1000
        } else {
            1
        }
    }
}

fn enumerate(graph: &Maze, location: Node, cost_so_far: i64, max_cost: i64, tried: &mut FxHashMap<Node, i64>, valid: &mut FxHashSet<(usize, usize)>) -> bool {
    // if we're standing at the goal, this is a valid path.
    if location.loc == graph.goal {
        valid.insert(location.loc);
        return true;
    }
    if let Some(attempt) = tried.get(&location) {
        if *attempt < cost_so_far {
            return false;
        }
    }
    // check if we can make it to the goal from here:
    let mut any_path = false;
    for child in graph.children(location) {
        let cost = graph.edge_weight(location, child);
        if cost_so_far + cost > max_cost {
            continue;
        }
        if enumerate(graph, child, cost_so_far + cost, max_cost, tried, valid) {
            valid.insert(location.loc);
            any_path = true;
        }
    }
    tried.insert(location, cost_so_far);
    any_path
}

pub fn task16() -> Result<AocResult<i64, usize>> {
    let task = std::hint::black_box(include_str!("../tasks/task16.txt"));

    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut path = FxHashSet::default();
    for (r, row) in task.lines().enumerate() {
        for (c, v) in row.as_bytes().iter().enumerate() {
            if b".SE".contains(v) {
                path.insert((r, c));
            }
            if *v == b'S' {
                start = (r, c);
            }
            if *v == b'E' {
                end = (r, c);
            }
        }
    }

    let graph = Maze {
        inner: path,
        root: Node {
            dir: Dir::E,
            loc: start,
        },
        goal: end,
    };

    let mut searcher = graphsearch::dijkstra::Dijkstra::new();
    searcher.search_tracked(&graph, graph.root);
    let path = searcher.path().unwrap();
    let cost = graph.path_cost(&path);

    let mut locations = FxHashSet::default();
    let mut tried = FxHashMap::default();
    enumerate(&graph, graph.root, 0, cost, &mut tried, &mut locations);

    let a = cost;
    let b = locations.len();

    Ok(AocResult { a, b })
}
