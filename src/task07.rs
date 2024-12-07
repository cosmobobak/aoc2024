use anyhow::{bail, Context, Result};
use arrayvec::ArrayVec;

use crate::AocResult;

fn seq_map<const OPCOUNT: u64>(mut mask: u64, seq: &[i16]) -> i64 {
    let mut vals = seq.iter();
    let mut acc = i64::from(*vals.next().unwrap());
    for val in vals {
        match mask % OPCOUNT {
            0 => acc *= i64::from(*val),
            1 => acc += i64::from(*val),
            2 => acc = (acc * (10i64.pow(val.ilog10() + 1))) + i64::from(*val),
            _ => unreachable!(),
        }
        mask /= OPCOUNT;
    }
    acc
}

fn combinations_match<const OPCOUNT: u64>(seq: &[i16], callback: impl Fn(i64) -> bool) -> bool {
    // one less than the sequence-length, because we
    // insert operators *between* values.
    let combinations = OPCOUNT.pow((seq.len() - 1).try_into().unwrap());
    let mask = combinations.next_power_of_two() - 1;
    let mut n = 0u64;
    // carry-rippler trick:
    for _ in 0..combinations {
        if callback(seq_map::<OPCOUNT>(n, seq)) {
            return true;
        }
        n = n.wrapping_sub(mask) & mask;
    }

    false
}

pub fn task07() -> Result<AocResult<i64, i64>> {
    let task = std::hint::black_box(include_str!("../tasks/task07.txt"));

    let mut makable1 = 0;
    let mut makable2 = 0;
    for line in task.lines() {
        let (target, sequence) = line.split_once(": ").context("no delimiter!")?;
        let target = target.parse::<i64>()?;
        let sequence = sequence
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<ArrayVec<i16, 32>, _>>()?;

        let match1 = combinations_match::<2>(&sequence, |v| v == target);
        let match2 = match1 || combinations_match::<3>(&sequence, |v| v == target);
        makable1 += i64::from(match1) * target;
        makable2 += i64::from(match2) * target;
    }

    Ok(AocResult {
        a: makable1,
        b: makable2,
    })
}
