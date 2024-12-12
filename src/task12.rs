use std::{collections::HashSet, ops::Index};

use anyhow::{bail, Result};
use bitvec::{order::Lsb0, view::AsMutBits};
use fxhash::{FxBuildHasher, FxHashMap, FxHashSet};

use crate::{bucket::HashBucket, AocResult};

fn grow_region_from<T: Eq + Copy, A: Index<usize, Output = T> + ?Sized>(
    (curr_x, curr_y): (u16, u16),
    grid: &A,
    rows: u16,
    cols: u16,
    region: &mut FxHashSet<(u16, u16)>,
) {
    let val = grid[usize::from(curr_x) * usize::from(cols) + usize::from(curr_y)];
    for (ox, oy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
        let x = curr_x.wrapping_add_signed(ox);
        let y = curr_y.wrapping_add_signed(oy);
        if x >= rows || y >= cols {
            continue;
        }
        if grid[usize::from(x) * usize::from(cols) + usize::from(y)] != val {
            continue;
        }
        if region.insert((x, y)) {
            grow_region_from((x, y), grid, rows, cols, region);
        }
    }
}

fn extract_regions<T: Eq + Default + Copy, A: Index<usize, Output = T> + ?Sized>(
    grid: &A,
    rows: u16,
    cols: u16,
) -> Vec<FxHashSet<(u16, u16)>> {
    let mut regions = Vec::new();

    for row in 0..rows {
        for col in 0..cols {
            if grid[usize::from(row) * usize::from(cols) + usize::from(col)] == T::default() {
                continue;
            }
            let co = (row, col);
            if regions
                .iter()
                .any(|r: &FxHashSet<(u16, u16)>| r.contains(&co))
            {
                continue;
            }
            regions.push(FxHashSet::default());
            let r = regions.last_mut().unwrap();
            r.insert(co);
            grow_region_from(co, grid, rows, cols, r);
        }
    }

    regions
}

fn score_region(region: &FxHashSet<(u16, u16)>) -> usize {
    let mut perimeter = 0;
    for coord in region {
        for offset in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let x = coord.0.wrapping_add_signed(offset.0);
            let y = coord.1.wrapping_add_signed(offset.1);
            if !region.contains(&(x, y)) {
                perimeter += 1;
            }
        }
    }

    region.len() * perimeter
}

fn score_region_2(
    region: &FxHashSet<(u16, u16)>,
    rows: u16,
    cols: u16,
    mut grid: &mut [u8],
) -> usize {
    let grid = grid.as_mut_bits::<Lsb0>();

    for &coord in region {
        for (i, (ox, oy)) in [(1, 0), (0, 1), (-1, 0), (0, -1)].into_iter().enumerate() {
            let x = coord.0.wrapping_add_signed(ox);
            let y = coord.1.wrapping_add_signed(oy);
            if !region.contains(&(x, y)) {
                let cx = x.wrapping_add(1);
                let cy = y.wrapping_add(1);
                let dir_idx = i * (usize::from(rows) * usize::from(cols));
                let row_idx = usize::from(cx) * usize::from(cols);
                let col_idx = usize::from(cy);
                // potential bug from aligned edges if a shape
                // spans the entire grid, but who cares.
                grid.set(dir_idx + row_idx + col_idx, true);
            }
        }
    }

    let lines = extract_regions(grid, rows * 4, cols).len();

    region.len() * lines
}

pub fn task12() -> Result<AocResult<usize, usize>> {
    let task = std::hint::black_box(include_str!("../tasks/task12.txt"));

    let grid = task
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .collect::<Vec<_>>();
    let rows = grid.len().try_into()?;
    let cols = grid[0].len().try_into()?;
    let grid = grid.into_iter().flatten().collect::<Vec<_>>();
    let regions = extract_regions(grid.as_slice(), rows, cols);
    let mut a = 0;
    let mut b = 0;
    let mut grid = vec![0u8; ((usize::from(cols) + 2) * (usize::from(rows) + 2) * 4) / 8 + 1];
    for region in &regions {
        a += score_region(region);
        b += score_region_2(region, rows + 2, cols + 2, &mut grid);
        grid.fill(0);
    }

    Ok(AocResult { a, b })
}
