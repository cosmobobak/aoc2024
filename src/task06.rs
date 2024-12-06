use std::collections::HashSet;

use anyhow::{Context, Result};
use arrayvec::ArrayVec;
use fxhash::FxHashSet;

use crate::{bucket::{ArrayBucket, HashBucket}, AocResult};

const CAP: usize = 32;

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
            .filter_map(move |(col, val)| (*val == b'#').then_some((u16::try_from(row), u16::try_from(col))))
    });

    let rows = u16::try_from(task.lines().count())?;
    let cols = u16::try_from(task.lines().next().unwrap().len())?;

    let mut row_map = ArrayBucket::<_, _, CAP>::new();
    let mut col_map = ArrayBucket::<_, _, CAP>::new();

    for (row, col) in obstacles {
        let row = row?;
        let col = col?;
        row_map.push(row, col);
        col_map.push(col, row);
    }

    let (guard_row, guard_col) = task
        .lines()
        .enumerate()
        .find_map(|(row, l)| {
            l.as_bytes()
                .iter()
                .enumerate()
                .find_map(|(col, v)| (*v == b'^').then_some((u16::try_from(row), u16::try_from(col))))
        })
        .context("couldn't find ^!")?;
    let guard_pos = (guard_row?, guard_col?);

    let mut states = FxHashSet::default();

    let mut seen = HashSet::new();
    exec::<false>(guard_pos, &col_map, &row_map, rows, cols, &mut states, |(row, col)| {
        seen.insert((row, col));
    });

    let mut successful_blockers = 0;
    // only makes sense to place a blocker on somewhere the guard actually walks
    for &(row, col) in &seen {
        states.clear();
        row_map.push(row, col);
        col_map.push(col, row);
        successful_blockers += usize::from(exec::<true>(
            guard_pos, &col_map, &row_map, rows, cols, &mut states, drop,
        ));
        row_map.remove(row, col);
        col_map.remove(col, row);
    }

    let a = seen.len();
    let b = successful_blockers;

    Ok(AocResult { a, b })
}

fn exec<const PART_2: bool>(
    mut guard_pos: (u16, u16),
    col_map: &ArrayBucket<u16, u16, CAP>,
    row_map: &ArrayBucket<u16, u16, CAP>,
    rows: u16,
    cols: u16,
    states: &mut FxHashSet<(Dir, u16, u16)>,
    mut acc: impl FnMut((u16, u16)),
) -> bool {
    let mut dir = Dir::Up;

    loop {
        if PART_2 && !states.insert((dir, guard_pos.0, guard_pos.1)) {
            return true; // repeat found
        }
        match dir {
            Up | Down => {
                let obstacles = col_map.find(&guard_pos.1).unwrap_or(&[]);
                let new_row = if dir == Up {
                    let Some(first_blocker) = obstacles.iter().copied().filter(|v| *v < guard_pos.0).max()
                    else {
                        break;
                    };
                    first_blocker + 1
                } else {
                    let Some(first_blocker) = obstacles.iter().copied().filter(|v| *v > guard_pos.0).min()
                    else {
                        break;
                    };
                    first_blocker - 1
                };
                let min = guard_pos.0.min(new_row);
                let max = guard_pos.0.max(new_row);
                for r in min..=max {
                    acc((r, guard_pos.1));
                }
                guard_pos.0 = new_row;
            }
            Left | Right => {
                let obstacles = row_map.find(&guard_pos.0).unwrap_or(&[]);
                let new_col = if dir == Left {
                    let Some(first_blocker) = obstacles.iter().copied().filter(|v| *v < guard_pos.1).max()
                    else {
                        break;
                    };
                    first_blocker + 1
                } else {
                    let Some(first_blocker) = obstacles.iter().copied().filter(|v| *v > guard_pos.1).min()
                    else {
                        break;
                    };
                    first_blocker - 1
                };
                let min = guard_pos.1.min(new_col);
                let max = guard_pos.1.max(new_col);
                for c in min..=max {
                    acc((guard_pos.0, c));
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
