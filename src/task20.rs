use std::fmt::Display;

use anyhow::{bail, Result};
use graphsearch::{
    dijkstra::Dijkstra,
    graph::{Graph, HeuristicGraph, WeightedGraph},
    graphsearcher::GraphSearcher,
};

use crate::AocResult;

#[derive(Debug)]
struct Grid {
    buf: Vec<u8>,
    width: usize,
    height: usize,
    root: usize,
    goal: usize,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        assert!(self.buf.len() == self.width * self.height);
        for block in self.buf.chunks_exact(self.width) {
            writeln!(f, "{}", std::str::from_utf8(block).unwrap())?;
        }
        Ok(())
    }
}

impl Graph for Grid {
    type Node = usize;
    type Edge = u8;

    fn root(&self) -> Self::Node {
        self.root
    }

    fn children(&self, node: Self::Node) -> Vec<Self::Node> {
        let ocol = node % self.width;
        [1, -1, self.width as isize, -(self.width as isize)]
            .into_iter()
            .filter_map(|offset| {
                let new = node.wrapping_add_signed(offset);
                let ncol = new % self.width;
                (new < self.buf.len() && usize::abs_diff(ocol, ncol) < 2 && self.buf[new] != b'#')
                    .then_some(new)
            })
            .collect()
    }

    fn edges(&self, node: Self::Node) -> Vec<Self::Edge> {
        let ocol = node % self.width;
        [1, -1, self.width as isize, -(self.width as isize)]
            .into_iter()
            .enumerate()
            .filter_map(|(i, offset)| {
                let new = node.wrapping_add_signed(offset);
                let ncol = new % self.width;
                (new < self.buf.len() && usize::abs_diff(ocol, ncol) < 2 && self.buf[new] != b'#')
                    .then_some(i as u8)
            })
            .collect()
    }

    fn is_goal(&self, node: Self::Node) -> bool {
        node == self.goal
    }
}

impl WeightedGraph for Grid {
    fn edge_weight(&self, _: Self::Node, _: Self::Node) -> i64 {
        1
    }
}

pub fn task20() -> Result<AocResult<i64, i64>> {
    let task = std::hint::black_box(include_str!("../tasks/task20.txt"));

    let mut buf = Vec::new();

    let mut width = 0;
    let mut height = 0;
    let mut root = None;
    let mut goal = None;
    for (row, line) in task.lines().enumerate() {
        buf.extend_from_slice(line.as_bytes());
        width = line.len();
        height += 1;
        if let Some(col) = line.find('S') {
            root = Some((row, col));
        }
        if let Some(col) = line.find('E') {
            goal = Some((row, col));
        }
    }

    let root = root.unwrap();
    let root = root.0 * width + root.1;
    let goal = goal.unwrap();
    let goal = goal.0 * width + goal.1;

    let mut grid = Grid {
        buf,
        width,
        height,
        root: goal,
        goal: usize::MAX,
    };

    // cleanup content now that we know the root / goal locations
    grid.buf[root] = b'.';
    grid.buf[goal] = b'.';

    let mut backsearcher = graphsearch::dijkstra::Dijkstra::new();
    // crawl backwards from the goal
    backsearcher.search_tracked(&grid, goal);
    let mut distances = vec![0i16; *backsearcher.distances().keys().max().unwrap() + 1];
    backsearcher.distances().iter().for_each(|(&k, &v)| {
        distances[k] = v.try_into().unwrap();
    });

    // vertical cuts
    let mut a = 0;
    let mut b = 0;
    do_leaps::<2>(width, height, &grid.buf, &distances, |saving| {
        a += i64::from(saving > 99);
    });
    do_leaps::<20>(width, height, &grid.buf, &distances, |saving| {
        b += i64::from(saving > 99);
    });

    Ok(AocResult { a, b })
}

fn do_leaps<const LEAP: isize>(
    width: usize,
    height: usize,
    buf: &[u8],
    distances: &[i16],
    mut callback: impl FnMut(i64),
) {
    for row in 1..height - 1 {
        for col in 1..width - 1 {
            let idx1 = row * width + col;
            if buf[idx1] != b'.' {
                continue;
            }

            for ox in -LEAP..LEAP + 1 {
                let nx = row.wrapping_add_signed(ox);
                if nx >= width {
                    continue;
                }
                let base = nx * width;
                for oy in -LEAP + ox.abs()..LEAP + 1 - ox.abs() {
                    if ox == 0 && oy == 0 {
                        continue;
                    }

                    let ny = col.wrapping_add_signed(oy);
                    if ny >= height {
                        continue;
                    }
                    let idx2 = base + ny;

                    if buf[idx2] != b'.' {
                        continue;
                    }

                    let saving = i64::from(distances[idx1])
                        - i64::from(distances[idx2])
                        - (ox.abs() + oy.abs()) as i64;

                    callback(saving);
                }
            }
        }
    }
}
