use anyhow::{bail, Context, Result};
use nom::AsChar;

use crate::AocResult;

// find the minimal ratio (by score) that will satisfy
// a * v1 + b * v2 = t. Returns None if no solution is possible.
const fn solve<const MAX: i64>(
    ((a1, a2), (b1, b2), (t1, t2)): ((i64, i64), (i64, i64), (i64, i64)),
) -> i64 {
    let determinant = a1 * b2 - a2 * b1;
    let v1 = (t1 * b2 - t2 * b1) / determinant;
    let v2 = (a1 * t2 - a2 * t1) / determinant;

    if v1 > MAX || v2 > MAX || a1 * v1 + b1 * v2 != t1 || a2 * v1 + b2 * v2 != t2 {
        return 0;
    }

    v1 * 3 + v2
}

#[allow(clippy::similar_names)]
pub fn task13() -> Result<AocResult<i64, i64>> {
    let task = std::hint::black_box(include_str!("../tasks/task13.txt"));

    let (a, b) = task
        .split("\n\n")
        .filter_map(|block| {
            let mut in_int = false;
            let mut start = 0;
            let [r1x, r1y, r2x, r2y, tx, ty] = block
                .as_bytes()
                .iter()
                .enumerate()
                .filter_map(|(i, b)| {
                    let digit = b.is_dec_digit();
                    if in_int && !digit {
                        in_int = false;
                        return Some(&block[start..i]);
                    } else if in_int && i + 1 == block.len() {
                        in_int = false;
                        return Some(&block[start..i + 1]);
                    } else if !in_int && digit {
                        start = i;
                        in_int = true;
                    }
                    None
                })
                .map(str::parse)
                .collect::<Result<Vec<_>, _>>()
                .ok()?
                .try_into()
                .ok()?;

            let a = (r1x, r1y);
            let b = (r2x, r2y);
            let t1 = (tx, ty);
            let t2 = (tx + 10_000_000_000_000, ty + 10_000_000_000_000);

            Some(((a, b, t1), (a, b, t2)))
        })
        .map(|(t1, t2)| (solve::<100>(t1), solve::<{ i64::MAX }>(t2)))
        .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));

    Ok(AocResult { a, b })
}
