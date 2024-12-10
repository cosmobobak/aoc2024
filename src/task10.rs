use anyhow::{bail, Result};
use arrayvec::ArrayVec;
use bitvec::{slice::BitSlice, view::AsMutBits};

use crate::AocResult;

fn score(
    current: u8,
    pos: (usize, usize),
    grid: &[ArrayVec<u8, 64>],
    seen: &mut BitSlice<u8>,
) -> (i32, i32) {
    let rows = grid.len();
    if current == b'9' {
        if seen[pos.0 + pos.1 * rows] {
            return (0, 1);
        }
        seen.set(pos.0 + pos.1 * rows, true);
        return (1, 1);
    }

    let cols = grid[0].len();
    let mut acc = (0, 0);
    for offset in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
        let x = pos.0.wrapping_add_signed(offset.0);
        let y = pos.1.wrapping_add_signed(offset.1);
        if x >= rows || y >= cols {
            continue;
        }
        let val = grid[x][y];
        if val == current + 1 {
            let (p1, p2) = score(val, (x, y), grid, seen);
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
    let mut seen = vec![0u8; rows * cols / 8 + 1];
    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == b'0' {
                seen.fill(0);
                let (p1, p2) = score(b'0', (row, col), &grid, seen.as_mut_bits());
                acc.0 += p1;
                acc.1 += p2;
            }
        }
    }

    Ok(AocResult { a: acc.0, b: acc.1 })
}
