use std::collections::HashSet;

use anyhow::{Context, Result};
use arrayvec::ArrayVec;

use crate::{bucket::Bucket, AocResult};

#[derive(PartialEq, Eq, Clone, Copy)]
enum Dir {
    Up, Right, Down, Left
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

pub fn task06() -> Result<AocResult<usize, i32>> {
    let task = std::hint::black_box(include_str!("../tasks/task06.txt"));

    let obstacles = task.lines().enumerate().flat_map(|(row, line)| {
        line.as_bytes()
            .iter()
            .enumerate()
            .filter_map(move |(col, val)| (*val == b'#').then_some((row, col)))
    });

    let mut row_map = Bucket::<_, _, 32>::new();
    let mut col_map = Bucket::<_, _, 32>::new();

    for (row, col) in obstacles {
        row_map.push(row, col);
        col_map.push(col, row);
    }

    let mut guard_pos = task.lines().enumerate().find_map(|(r, l)| {
        l.as_bytes()
            .iter()
            .enumerate()
            .find_map(|(c, v)| (*v == b'^').then_some((r, c)))
    }).context("couldn't find ^!")?;

    let mut dir = Dir::Up;
    let mut total = 0;

    loop {
        match dir {
            Up | Down => {
                let row = guard_pos.0;
                let col = guard_pos.1;
                let obstacles = col_map.find(&col).unwrap_or(&[]);
                let new_row = if dir == Up {
                    let Some(first_blocker) = obstacles.iter().copied().filter(|v| *v < row).max() else {
                        break;
                    };
                    first_blocker + 1
                } else {
                    let Some(first_blocker) = obstacles.iter().copied().filter(|v| *v > row).min() else {
                        break;
                    };
                    first_blocker - 1
                };
                total += guard_pos.0.abs_diff(new_row);
                guard_pos.0 = new_row;
            },
            Left | Right => {
                let row = guard_pos.0;
                let col = guard_pos.1;
                let obstacles = row_map.find(&row).unwrap_or(&[]);
                let new_col = if dir == Left {
                    let Some(first_blocker) = obstacles.iter().copied().filter(|v| *v < col).max() else {
                        break;
                    };
                    first_blocker + 1
                } else {
                    let Some(first_blocker) = obstacles.iter().copied().filter(|v| *v > col).min() else {
                        break;
                    };
                    first_blocker - 1
                };
                total += guard_pos.1.abs_diff(new_col);
                guard_pos.1 = new_col;
            },
        };
        dir = dir.next();
    }

    let a = total;
    let b = 0;

    Ok(AocResult { a, b })
}
