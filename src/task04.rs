use anyhow::Result;

use crate::AocResult;

fn raycast<const X_INC: isize, const Y_INC: isize>(
    x: isize,
    y: isize,
) -> impl Iterator<Item = (isize, isize)> {
    (0..4).map(move |i| (x + X_INC * i, y + Y_INC * i))
}

fn seq(is: impl Iterator<Item = (isize, isize)>, grid: &[&[u8]]) -> Option<[u8; 4]> {
    let mut v = [0; 4];
    for ((x, y), out) in is.zip(&mut v) {
        let r = grid.get(usize::try_from(x).ok()?)?;
        *out = *r.get(usize::try_from(y).ok()?)?;
    }
    Some(v)
}

pub fn task04() -> Result<AocResult<i32, i32>> {
    let task = std::hint::black_box(include_bytes!("../tasks/task04.txt"));

    let grid = task.split(|b| *b == b'\n').collect::<Vec<_>>();

    let mut sum1 = 0;

    let needle = Some(*b"XMAS");
    for (row, r) in grid.iter().zip(0..) {
        for (_, c) in row.iter().zip(0..) {
            sum1 += i32::from(seq(raycast::<1, 0>(r, c), &grid) == needle);
            sum1 += i32::from(seq(raycast::<0, 1>(r, c), &grid) == needle);
            sum1 += i32::from(seq(raycast::<-1, 0>(r, c), &grid) == needle);
            sum1 += i32::from(seq(raycast::<0, -1>(r, c), &grid) == needle);
            sum1 += i32::from(seq(raycast::<1, 1>(r, c), &grid) == needle);
            sum1 += i32::from(seq(raycast::<1, -1>(r, c), &grid) == needle);
            sum1 += i32::from(seq(raycast::<-1, -1>(r, c), &grid) == needle);
            sum1 += i32::from(seq(raycast::<-1, 1>(r, c), &grid) == needle);
        }
    }

    let mut sum2 = 0;

    // for each sequence of three rows:
    for row3 in grid.windows(3) {
        // for each window of three values:
        for ((a, b), c) in row3[0]
            .windows(3)
            .zip(row3[1].windows(3))
            .zip(row3[2].windows(3))
        {
            // check if it matches the desired patterns:
            let diag1 = [a[0], b[1], c[2]];
            let diag2 = [a[2], b[1], c[0]];
            if (diag1 == *b"MAS" || diag1 == *b"SAM") && (diag2 == *b"MAS" || diag2 == *b"SAM") {
                sum2 += 1;
            }
        }
    }

    Ok(AocResult { a: sum1, b: sum2 })
}
