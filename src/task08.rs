use std::collections::HashSet;

use anyhow::{bail, Result};

use crate::{bucket::ArrayBucket, AocResult};

type Float = f64;

// kept around for posterity because i feel stupid but liked this approach
// to the problem i made up in my head.
#[allow(
    clippy::suboptimal_flops,
    clippy::cast_lossless,
    clippy::cast_possible_truncation
)]
fn _visit_around_circle(
    center: (Float, Float),
    radius: Float,
    visitor: &mut impl FnMut((i32, i32)),
) {
    let (cx, cy) = center;
    let r2 = radius * radius;

    // Bounding box around the circle
    let x_min = (cx - radius).ceil() as i32;
    let x_max = (cx + radius).floor() as i32;
    let y_min = (cy - radius).ceil() as i32;
    let y_max = (cy + radius).floor() as i32;

    for x in x_min..x_max + 1 {
        for y in y_min..y_max + 1 {
            // Check if (x, y) lies on the circle
            let dx = x as f64 - cx;
            let dy = y as f64 - cy;
            if (dx * dx + dy * dy - r2).abs() < 1e-6 {
                visitor((x, y));
            }
        }
    }
}

#[allow(clippy::cast_possible_truncation)]
fn visit_all_along_line<const PART_1: bool>(
    mut a: (i32, i32),
    mut b: (i32, i32),
    max_x: i32,
    max_y: i32,
    mut visitor: impl FnMut((i32, i32)),
) {
    let a_to_b = (b.0 - a.0, b.1 - a.1);
    // downwards loop:
    if PART_1 {
        // skip1
        a = (a.0 - a_to_b.0, a.1 - a_to_b.1);
    }
    while (0..max_x).contains(&a.0) && (0..max_y).contains(&a.1) {
        visitor(a);
        a = (a.0 - a_to_b.0, a.1 - a_to_b.1);
        if PART_1 {
            break;
        }
    }
    // upwards loop
    if PART_1 {
        // skip1
        b = (b.0 + a_to_b.0, b.1 + a_to_b.1);
    }
    while (0..max_x).contains(&b.0) && (0..max_y).contains(&b.1) {
        visitor(b);
        b = (b.0 + a_to_b.0, b.1 + a_to_b.1);
        if PART_1 {
            break;
        }
    }
}

pub fn task08() -> Result<AocResult<usize, usize>> {
    let task = std::hint::black_box(include_str!("../tasks/task08.txt"));

    let rows = task.lines().count().try_into()?;
    let cols = task.lines().next().unwrap().len().try_into()?;

    let mut nodes = ArrayBucket::<u8, (i32, i32), 32, false>::new();

    for (row, line) in task.lines().enumerate() {
        for (col, val) in line.as_bytes().iter().enumerate() {
            if *val != b'.' {
                nodes.push(*val, (row.try_into()?, col.try_into()?));
            }
        }
    }

    let mut antinodes1 = HashSet::new();
    let mut antinodes2 = HashSet::new();

    for (_, towers) in nodes.iter() {
        for a in 0..towers.len() {
            for b in a + 1..towers.len() {
                visit_all_along_line::<true>(towers[a], towers[b], rows, cols, |point| {
                    antinodes1.insert(point);
                });
                visit_all_along_line::<false>(towers[a], towers[b], rows, cols, |point| {
                    antinodes2.insert(point);
                });
            }
        }
    }

    Ok(AocResult {
        a: antinodes1.len(),
        b: antinodes2.len(),
    })
}
