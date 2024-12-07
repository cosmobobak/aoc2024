use anyhow::{bail, Context, Result};
use arrayvec::ArrayVec;

use crate::AocResult;

fn seq_map<const OPCOUNT: u64>(guard: u64, mut mask: u64, seq: &[u16]) -> u64 {
    let mut vals = seq.iter();
    let mut acc = u64::from(*vals.next().unwrap());

    for &val in vals {
        if acc > guard {
            // early exit hopefully useful for cutting search space,
            // valid since ops induce a strictly-increasing accumulator
            break;
        }
        match mask % OPCOUNT {
            0 => acc *= u64::from(val),
            1 => acc += u64::from(val),
            2 => acc = (acc * (10u64.pow(val.ilog10() + 1))) + u64::from(val),
            _ => unreachable!(),
        }
        mask /= OPCOUNT;
    }

    acc
}

fn combinations_match<const OPCOUNT: u64>(seq: &[u16], target: u64) -> bool {
    // one less than the sequence-length, because we
    // insert operators *between* values.
    let combinations = OPCOUNT.pow((seq.len() - 1).try_into().unwrap());
    let mask = combinations.next_power_of_two() - 1;
    let mut n = 0u64;
    // carry-rippler trick:
    for _ in 0..combinations {
        if seq_map::<OPCOUNT>(target, n, seq) == target {
            return true;
        }
        n = n.wrapping_sub(mask) & mask;
    }

    false
}

pub fn task07() -> Result<AocResult<u64, u64>> {
    let task = std::hint::black_box(include_str!("../tasks/task07.txt"));

    let mut makable1 = 0;
    let mut makable2 = 0;
    for line in task.lines() {
        let (target, sequence) = line.split_once(": ").context("no delimiter!")?;
        let target = target.parse::<u64>()?;
        let sequence = sequence
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<ArrayVec<u16, 32>, _>>()?;

        let match1 = combinations_match::<2>(&sequence, target);
        let match2 = match1 || combinations_match::<3>(&sequence, target);
        makable1 += u64::from(match1) * target;
        makable2 += u64::from(match2) * target;
    }

    Ok(AocResult {
        a: makable1,
        b: makable2,
    })
}
