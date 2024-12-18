use std::collections::HashSet;

use anyhow::{Context, Result};
use arrayvec::ArrayVec;
use bitvec::{ptr::BitRef, slice::BitSlice, view::BitView};
use fxhash::FxHashSet;

use crate::{
    bucket::{ArrayBucket, HashBucket},
    AocResult,
};

const CAP: usize = 32;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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

// so i can play with bit widths
type Uint = u8;

pub fn task06() -> Result<AocResult<usize, usize>> {
    let task = std::hint::black_box(include_str!("../tasks/task06.txt"));

    let obstacles = task.lines().enumerate().flat_map(|(row, line)| {
        line.as_bytes()
            .iter()
            .enumerate()
            .filter_map(move |(col, val)| {
                (*val == b'#').then_some((Uint::try_from(row), Uint::try_from(col)))
            })
    });

    let rows = Uint::try_from(task.lines().count())?;
    let cols = Uint::try_from(task.lines().next().unwrap().len())?;

    let mut row_map = HashBucket::<_, _, CAP, true>::new();
    let mut col_map = HashBucket::<_, _, CAP, true>::new();

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
            l.as_bytes().iter().enumerate().find_map(|(col, v)| {
                (*v == b'^').then_some((Uint::try_from(row), Uint::try_from(col)))
            })
        })
        .context("couldn't find ^!")?;
    let guard_pos = (guard_row?, guard_col?);

    let mut states = vec![0u8; rows as usize * cols as usize * 4 / 8];

    let mut seen = HashSet::new();
    exec::<false>(
        guard_pos,
        &col_map,
        &row_map,
        rows,
        cols,
        states.view_bits_mut(),
        |(row, col)| {
            seen.insert((row, col));
        },
    );
    seen.remove(&guard_pos);

    let mut successful_blockers = 0;
    // only makes sense to place a blocker on somewhere the guard actually walks
    for &(row, col) in &seen {
        states.fill(0);
        row_map.push(row, col);
        col_map.push(col, row);
        successful_blockers += usize::from(exec::<true>(
            guard_pos,
            &col_map,
            &row_map,
            rows,
            cols,
            states.view_bits_mut(),
            drop,
        ));
        row_map.remove(row, col);
        col_map.remove(col, row);
    }

    let a = seen.len() + 1;
    let b = successful_blockers;

    Ok(AocResult { a, b })
}

fn exec<const PART_2: bool>(
    mut guard: (Uint, Uint),
    col_map: &HashBucket<Uint, Uint, CAP, true>,
    row_map: &HashBucket<Uint, Uint, CAP, true>,
    rows: Uint,
    cols: Uint,
    states: &mut BitSlice<u8>,
    mut acc: impl FnMut((Uint, Uint)),
) -> bool {
    let mut dir = Dir::Up;

    loop {
        let idx1 = usize::from(guard.0);
        let idx2 = usize::from(guard.1);
        let idx3 = dir as usize;
        let idx = idx1 + idx2 * usize::from(rows) + idx3 * usize::from(rows) * usize::from(cols);
        if PART_2 && states[idx] {
            return true; // repeat found
        }
        states.set(idx, true);
        match dir {
            Up | Down => {
                let Some(blockers) = col_map.find(&guard.1) else {
                    break;
                };
                let blocker_idx = blockers
                    .iter()
                    .position(|&v| v > guard.0)
                    .unwrap_or(blockers.len());
                let new_row = if dir == Up {
                    if blocker_idx == 0 {
                        break;
                    }
                    let new = blockers[blocker_idx - 1] + 1;
                    for r in new..guard.0 + 1 {
                        acc((r, guard.1));
                    }
                    new
                } else {
                    if blocker_idx == blockers.len() {
                        break;
                    }
                    let new = blockers[blocker_idx] - 1;
                    for r in guard.0..new + 1 {
                        acc((r, guard.1));
                    }
                    new
                };
                guard.0 = new_row;
            }
            Left | Right => {
                let Some(blockers) = row_map.find(&guard.0) else {
                    break;
                };
                let blocker_idx = blockers
                    .iter()
                    .position(|&v| v > guard.1)
                    .unwrap_or(blockers.len());
                let new_col = if dir == Left {
                    if blocker_idx == 0 {
                        break;
                    }
                    let new = blockers[blocker_idx - 1] + 1;
                    for c in new..guard.1 + 1 {
                        acc((guard.0, c));
                    }
                    new
                } else {
                    if blocker_idx == blockers.len() {
                        break;
                    }
                    let new = blockers[blocker_idx] - 1;
                    for c in guard.1..new + 1 {
                        acc((guard.0, c));
                    }
                    new
                };
                guard.1 = new_col;
            }
        };
        dir = dir.next();
    }

    // exit cleanup, maybe improvable
    match dir {
        Up => {
            for row in 0..guard.0 {
                acc((row, guard.1));
            }
        }
        Down => {
            for row in guard.0..rows {
                acc((row, guard.1));
            }
        }
        Left => {
            for col in 0..guard.1 {
                acc((guard.0, col));
            }
        }
        Right => {
            for col in guard.1..cols {
                acc((guard.0, col));
            }
        }
    }

    false // no repeat found
}
