use std::collections::HashSet;

use anyhow::{Context, Result};
use arrayvec::ArrayVec;

use crate::{bucket::Bucket, AocResult};

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

use Dir::{Down, Left, Right, Up};

impl Dir {
    const fn next(self) -> Self {
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
}

pub fn task06() -> Result<AocResult<usize, usize>> {
    let task = std::hint::black_box(include_str!("../tasks/task06.txt"));

    let obstacles = task.lines().enumerate().flat_map(|(row, line)| {
        line.as_bytes()
            .iter()
            .enumerate()
            .filter_map(move |(col, val)| (*val == b'#').then_some((row, col)))
    });

    let rows = task.lines().count();
    let cols = task.lines().next().unwrap().len();

    let mut row_map = Bucket::<_, _, 32>::new();
    let mut col_map = Bucket::<_, _, 32>::new();

    for (row, col) in obstacles {
        row_map.push(row, col);
        col_map.push(col, row);
    }

    let guard_pos = task
        .lines()
        .enumerate()
        .find_map(|(r, l)| {
            l.as_bytes()
                .iter()
                .enumerate()
                .find_map(|(c, v)| (*v == b'^').then_some((r, c)))
        })
        .context("couldn't find ^!")?;

    let mut seen = HashSet::new();
    exec::<false>(guard_pos, &col_map, &row_map, rows, cols, |(row, col)| {
        seen.insert((row, col));
    });

    let mut successful_blockers = 0;
    // only makes sense to place a blocker on somewhere the guard actually walks 
    for &(row, col) in &seen {
        row_map.push(row, col);
        col_map.push(col, row);
        successful_blockers += usize::from(exec::<true>(
            guard_pos, &col_map, &row_map, rows, cols, drop,
        ));
        row_map.remove(row, col);
        col_map.remove(col, row);
    }

    let a = seen.len();
    let b = successful_blockers;

    Ok(AocResult { a, b })
}

fn exec<const PART_2: bool>(
    mut guard_pos: (usize, usize),
    col_map: &Bucket<usize, usize, 32>,
    row_map: &Bucket<usize, usize, 32>,
    rows: usize,
    cols: usize,
    mut acc: impl FnMut((usize, usize)),
) -> bool {
    let mut dir = Dir::Up;
    let mut states = HashSet::new();

    loop {
        match dir {
            Up | Down => {
                let row = guard_pos.0;
                let col = guard_pos.1;
                let obstacles = col_map.find(&col).unwrap_or(&[]);
                let new_row = if dir == Up {
                    let Some(first_blocker) = obstacles.iter().copied().filter(|v| *v < row).max()
                    else {
                        break;
                    };
                    first_blocker + 1
                } else {
                    let Some(first_blocker) = obstacles.iter().copied().filter(|v| *v > row).min()
                    else {
                        break;
                    };
                    first_blocker - 1
                };
                let min = guard_pos.0.min(new_row);
                let max = guard_pos.0.max(new_row);
                for r in min..=max {
                    if PART_2 && !states.insert((dir, r, col)) {
                        return true; // repeat found
                    }

                    acc((r, col));
                }
                guard_pos.0 = new_row;
            }
            Left | Right => {
                let row = guard_pos.0;
                let col = guard_pos.1;
                let obstacles = row_map.find(&row).unwrap_or(&[]);
                let new_col = if dir == Left {
                    let Some(first_blocker) = obstacles.iter().copied().filter(|v| *v < col).max()
                    else {
                        break;
                    };
                    first_blocker + 1
                } else {
                    let Some(first_blocker) = obstacles.iter().copied().filter(|v| *v > col).min()
                    else {
                        break;
                    };
                    first_blocker - 1
                };
                let min = guard_pos.1.min(new_col);
                let max = guard_pos.1.max(new_col);
                for c in min..=max {
                    if PART_2 && !states.insert((dir, row, c)) {
                        return true; // repeat found
                    }

                    acc((row, c));
                }
                guard_pos.1 = new_col;
            }
        };
        dir = dir.next();
    }

    // exit cleanup, maybe improvable
    match dir {
        Up => {
            for row in 0..guard_pos.0 {
                acc((row, guard_pos.1));
            }
        }
        Down => {
            for row in guard_pos.0..rows {
                acc((row, guard_pos.1));
            }
        }
        Left => {
            for col in 0..guard_pos.1 {
                acc((guard_pos.0, col));
            }
        }
        Right => {
            for col in guard_pos.1..cols {
                acc((guard_pos.0, col));
            }
        }
    }

    false // no repeat found
}
