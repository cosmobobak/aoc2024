use anyhow::{bail, Context, Result};

use crate::AocResult;

#[derive(Clone, Copy)]
struct Task {
    a: (i64, i64),
    b: (i64, i64),
    t: (i64, i64),
}

impl Task {
    // find the minimal ratio (by score) that will satisfy
    // a * v1 + b * v2 = t. Returns None if no solution is possible.
    const fn minimal<const MAX: i64>(self) -> Option<(i64, i64)> {
        let (a1, a2) = self.a;
        let (b1, b2) = self.b;
        let (t1, t2) = self.t;

        let determinant = a1 * b2 - a2 * b1;
        let v1 = (t1 * b2 - t2 * b1) / determinant;
        let v2 = (a1 * t2 - a2 * t1) / determinant;

        if v1 > MAX || v2 > MAX {
            return None;
        }

        if a1 * v1 + b1 * v2 != t1 || a2 * v1 + b2 * v2 != t2 {
            return None;
        }

        Some((v1, v2))
    }
}

#[allow(clippy::similar_names)]
pub fn task13() -> Result<AocResult<i64, i64>> {
    let task = std::hint::black_box(include_str!("../tasks/task13.txt"));

    let (a, b) = task
        .split("\n\n")
        .filter_map(|block| {
            let [rule1, rule2, target] = block.lines().collect::<Vec<_>>()[..] else {
                return None;
            };

            let rule1 = rule1.split_once(": ")?.1;
            let rule2 = rule2.split_once(": ")?.1;
            let target = target.split_once(": ")?.1;

            let (r1x, r1y) = rule1.split_once(", ")?;
            let (r2x, r2y) = rule2.split_once(", ")?;
            let (tx, ty) = target.split_once(", ")?;

            let r1x = r1x.trim_start_matches(['X', '+']).parse().ok()?;
            let r1y = r1y.trim_start_matches(['Y', '+']).parse().ok()?;
            let r2x = r2x.trim_start_matches(['X', '+']).parse().ok()?;
            let r2y = r2y.trim_start_matches(['Y', '+']).parse().ok()?;
            let tx = tx.trim_start_matches(['X', '=']).parse().ok()?;
            let ty = ty.trim_start_matches(['Y', '=']).parse().ok()?;

            let a = (r1x, r1y);
            let b = (r2x, r2y);
            let t1 = (tx, ty);
            let t2 = (tx + 10_000_000_000_000, ty + 10_000_000_000_000);

            Some((Task { a, b, t: t1 }, Task { a, b, t: t2 }))
        })
        .map(|(t1, t2)| {
            (
                Task::minimal::<100>(t1).unwrap_or((0, 0)),
                Task::minimal::<{ i64::MAX }>(t2).unwrap_or((0, 0)),
            )
        })
        .map(|((a1, b1), (a2, b2))| ((a1 * 3 + b1), (a2 * 3 + b2)))
        .fold((0i64, 0i64), |acc, x| (acc.0 + x.0, acc.1 + x.1));

    Ok(AocResult { a, b })
}
