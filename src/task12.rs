use std::collections::HashSet;

use anyhow::{bail, Result};
use fxhash::{FxBuildHasher, FxHashMap, FxHashSet};

use crate::{bucket::HashBucket, AocResult};

fn grow_region_from(co: (usize, usize), grid: &[Vec<u8>], region: &mut FxHashSet<(usize, usize)>) {
    let rows = grid.len();
    let cols = grid[0].len();
    let val = grid[co.0][co.1];
    for offset in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
        let x = co.0.wrapping_add_signed(offset.0);
        let y = co.1.wrapping_add_signed(offset.1);
        if x >= rows || y >= cols {
            continue;
        }
        if grid[x][y] != val {
            continue;
        }
        if region.insert((x, y)) {
            grow_region_from((x, y), grid, region);
        }
    }
}

fn extract_regions(grid: &[Vec<u8>]) -> Vec<FxHashSet<(usize, usize)>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut regions = Vec::new();

    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == 0 {
                continue;
            }
            let co = (row, col);
            if regions.iter().any(|r: &FxHashSet<(usize, usize)>| r.contains(&co)) {
                continue;
            }
            regions.push(FxHashSet::default());
            let r= regions.last_mut().unwrap();
            r.insert(co);
            grow_region_from(co, grid, r);
        }
    }

    regions
}

fn score_region(region: &FxHashSet<(usize, usize)>) -> usize {
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

fn score_region_2(region: &FxHashSet<(usize, usize)>) -> usize {
    let mut perimeter = FxHashSet::default();
    let mut max_x = 0;
    let mut max_y = 0;
    for &coord in region {
        for offset in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let x = coord.0.wrapping_add_signed(offset.0);
            let y = coord.1.wrapping_add_signed(offset.1);
            if !region.contains(&(x, y)) {
                let cx = x.wrapping_add(1);
                let cy = y.wrapping_add(1);
                perimeter.insert(((cx, cy), offset));
                max_x = max_x.max(cx);
                max_y = max_y.max(cy);
            }
        }
    }

    // construct a grid:
    let mut grid = vec![vec![vec![0u8; max_y + 1]; max_x + 1]; 4];
    for &((x, y), offset) in &perimeter {
        match offset {
            (1, 0) => grid[0][x][y] = 1,
            (0, 1) => grid[1][x][y] = 1,
            (-1, 0) => grid[2][x][y] = 1,
            (0, -1) => grid[3][x][y] = 1,
            _ => unreachable!(),
        };
    }
    let mut regions = extract_regions(&grid[0]);
    regions.extend_from_slice(&extract_regions(&grid[1]));
    regions.extend_from_slice(&extract_regions(&grid[2]));
    regions.extend_from_slice(&extract_regions(&grid[3]));

    dbg!(dbg!(region.len()) * dbg!(regions.len()))
}

pub fn task12() -> Result<AocResult<usize, usize>> {
    let task = std::hint::black_box(include_str!("../tasks/task12.txt"));

    let grid = task.lines().map(|l| l.as_bytes().to_vec()).collect::<Vec<_>>();
    let regions = extract_regions(&grid);
    let mut a = 0;
    let mut b = 0;
    for region in &regions {
        a += score_region(region);
        b += score_region_2(region);
    }

    Ok(AocResult { a, b })
}
