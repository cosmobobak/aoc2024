use anyhow::{bail, Result};

use crate::AocResult;

pub fn task01() -> Result<AocResult<u32, i32>> {
    let task = std::hint::black_box(include_str!("../tasks/task01.txt"));

    let mut ls = Vec::with_capacity(1000);
    let mut rs = Vec::with_capacity(1000);
    let mut keys = Vec::with_capacity(1000);
    let mut vals = Vec::with_capacity(1000);

    for line in task.lines() {
        let [l, r] = line
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<Vec<i32>, _>>()?[..]
        else {
            bail!("unreachable code.");
        };

        ls.push(l);
        rs.push(r);
    }

    for &r in &rs {
        if let Some(i) = keys.iter().position(|&k| k == r) {
            vals[i] += 1;
        } else {
            keys.push(r);
            vals.push(1);
        }
    }

    ls.sort_unstable();
    rs.sort_unstable();

    let diff_sum = ls
        .iter()
        .zip(&rs)
        .map(|(&a, &b)| i32::abs_diff(a, b))
        .sum::<u32>();

    let count_sum = ls
        .iter()
        .map(|&v| v * keys.iter().position(|&k| k == v).map_or(0, |i| vals[i]))
        .sum::<i32>();

    Ok(AocResult {
        a: diff_sum,
        b: count_sum,
    })
}
