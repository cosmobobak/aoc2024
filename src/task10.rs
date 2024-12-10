use anyhow::{bail, Result};
use arrayvec::ArrayVec;
use fxhash::{FxHashMap, FxHashSet};

use crate::AocResult;

fn score(
    current: u8,
    pos: (usize, usize),
    (rows, cols): (usize, usize),
    grid: &[ArrayVec<u8, 64>],
    seen: &mut FxHashSet<(usize, usize)>,
) -> (i32, i32) {
    if current == b'9' {
        if !seen.insert(pos) {
            return (0, 1);
        }
        return (1, 1);
    }
    let locs = [(1, 0), (0, 1), (-1, 0), (0, -1)]
        .into_iter()
        .filter_map(|offset| {
            let x = pos.0.wrapping_add_signed(offset.0);
            let y = pos.1.wrapping_add_signed(offset.1);
            ((0..rows).contains(&x) && (0..cols).contains(&y)).then_some((x, y))
        });
    let mut acc = (0, 0);
    for loc in locs {
        let val = grid[loc.0][loc.1];
        if val == current + 1 {
            let (p1, p2) = score(val, loc, (rows, cols), grid, seen);
            acc.0 += p1;
            acc.1 += p2;
        }
    }
    acc
}

pub fn task10() -> Result<AocResult<i32, i32>> {
    let task = std::hint::black_box(include_str!("../tasks/task10.txt"));

    let grid = task
        .lines()
        .map(|l| l.as_bytes().iter().copied().collect::<ArrayVec<_, 64>>())
        .collect::<Vec<_>>();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut acc = (0, 0);
    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == b'0' {
                let mut seen = FxHashSet::default();
                let (p1, p2) = score(b'0', (row, col), (rows, cols), &grid, &mut seen);
                acc.0 += p1;
                acc.1 += p2;
            }
        }
    }

    Ok(AocResult { a: acc.0, b: acc.1 })
}
