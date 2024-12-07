use anyhow::{bail, Context, Result};

use crate::AocResult;

fn seq_map(mut mask: u64, seq: &[i32]) -> i64 {
    let mut vals = seq.iter();
    let mut acc = i64::from(*vals.next().unwrap_or(&0));
    for val in vals {
        if (mask & 1) == 0 {
            acc *= i64::from(*val);
        } else {
            acc += i64::from(*val);
        }
        mask >>= 1;
    }
    acc
}

fn combinations_match(seq: &[i32], callback: impl Fn(i64) -> bool) -> bool {
    // bitset of one less than the sequence-length because we
    // insert operators *between* values.
    let mask = (1u64 << (seq.len() - 1)) - 1;
    let mut n = 0u64;
    loop {
        if callback(seq_map(n, seq)) {
            break true;
        }
        n = (n.wrapping_sub(mask)) & mask;
        if n == 0 {
            break false;
        }
    }
}

pub fn task07() -> Result<AocResult<i64, i32>> {
    let task = std::hint::black_box(include_str!("../tasks/task07.txt"));

    let mut makable = 0;
    for line in task.lines() {
        let (target, sequence) = line.split_once(": ").context("no delimiter!")?;
        let target = target.parse::<i64>()?;
        let sequence = sequence
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<Vec<i32>, _>>()?;
        // println!("trying {target}: {sequence:?}");
        if combinations_match(&sequence, |v| v == target) {
            makable += target;
        }
    }

    Ok(AocResult { a: makable, b: 0 })
}
