use anyhow::{Context, Result};

use arrayvec::ArrayVec;

use crate::AocResult;

pub fn task02() -> Result<AocResult<i32, i32>> {
    let task = std::hint::black_box(include_str!("../tasks/task02.txt"));

    let (a, b) = task.lines().try_fold((0, 0), |(a, b), line| {
        let nums = line
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<ArrayVec<i16, 16>, _>>()?;

        let mut eliminations = (0..=nums.len()).rev();

        let (solvable, skip_unused) = loop {
            let Some(elimination) = eliminations.next() else {
                break (false, false);
            };

            let mut sequence = nums
                .iter()
                .copied()
                .enumerate()
                .filter_map(|(i, n)| (i != elimination).then_some(n));
            let first = sequence.next().context("No first element!")?;

            let (inc, dec, stable, _) =
                sequence.fold((true, true, true, first), |(inc, dec, stable, prev), n| {
                    (
                        inc && n > prev,
                        dec && n < prev,
                        stable && prev.abs_diff(n) <= 3,
                        n,
                    )
                });

            if (inc || dec) && stable {
                break (true, elimination == nums.len());
            }
        };

        Ok::<_, anyhow::Error>((
            a + i32::from(solvable && skip_unused),
            b + i32::from(solvable),
        ))
    })?;

    Ok(AocResult { a, b })
}
