#![allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]

use std::fmt::Display;

use anyhow::{bail, Result};
use graphsearch::{
    graph::{Graph, HeuristicGraph, WeightedGraph},
    graphsearcher::GraphSearcher,
};

use crate::AocResult;

const DIM: usize = 71;

struct Grid {
    buf: Vec<u8>,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        assert!(self.buf.len() == DIM.pow(2));
        for block in self.buf.chunks_exact(DIM) {
            writeln!(f, "{}", std::str::from_utf8(block).unwrap())?;
        }
        Ok(())
    }
}

impl Graph for Grid {
    type Node = usize;
    type Edge = u8;

    fn root(&self) -> Self::Node {
        todo!()
    }

    fn children(&self, node: Self::Node) -> Vec<Self::Node> {
        let ocol = node % DIM;
        [1, -1, DIM as isize, -(DIM as isize)]
            .into_iter()
            .filter_map(|offset| {
                let new = node.wrapping_add_signed(offset);
                let ncol = new % DIM;
                (new < self.buf.len() && usize::abs_diff(ocol, ncol) < 2 && self.buf[new] != b'#')
                    .then_some(new)
            })
            .collect()
    }

    fn edges(&self, node: Self::Node) -> Vec<Self::Edge> {
        let ocol = node % DIM;
        [1, -1, DIM as isize, -(DIM as isize)]
            .into_iter()
            .enumerate()
            .filter_map(|(i, offset)| {
                let new = node.wrapping_add_signed(offset);
                let ncol = new % DIM;
                (new < self.buf.len() && usize::abs_diff(ocol, ncol) < 2 && self.buf[new] != b'#')
                    .then_some(i as u8)
            })
            .collect()
    }

    fn is_goal(&self, node: Self::Node) -> bool {
        node == self.buf.len() - 1
    }
}

impl WeightedGraph for Grid {
    fn edge_weight(&self, _: Self::Node, _: Self::Node) -> i64 {
        1
    }
}

impl HeuristicGraph for Grid {
    fn heuristic(&self, node: Self::Node) -> i64 {
        let row = node / DIM;
        let col = node % DIM;
        // time to go is at least
        ((DIM - row) + (DIM - col)) as i64
    }
}

fn is_solvable(grid: &Grid) -> bool {
    let mut searcher = graphsearch::astar::AStar::new();
    searcher.search_tracked(grid, 0).is_some()
}

pub fn task18() -> Result<AocResult<i64, String>> {
    let task = std::hint::black_box(include_str!("../tasks/task18.txt"));

    let mut grid = Grid {
        buf: vec![b'.'; DIM.pow(2)],
    };

    let mut a = 0;
    let mut b = String::new();
    for (i, line) in task.lines().enumerate() {
        let (x, y) = line.split_once(',').unwrap();
        let row = y.parse::<usize>()?;
        let col = x.parse::<usize>()?;
        grid.buf[col + row * DIM] = b'#';
        if i == 1023 {
            let mut searcher = graphsearch::astar::AStar::new();
            searcher.search_tracked(&grid, 0);
            let path = searcher.path().unwrap();
            a = grid.path_cost(&path);
        }
        if !is_solvable(&grid) {
            b = format!("{x},{y}");
            break;
        }
    }

    Ok(AocResult { a, b })
}
