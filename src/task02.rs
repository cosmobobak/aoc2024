use anyhow::{Context, Result};

use arrayvec::ArrayVec;

pub fn task02() -> Result<()> {
    let start = std::time::Instant::now();
    let task = include_str!("../tasks/task02.txt");

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

    println!("Part 1: {a}");
    println!("Part 2: {b}");

    let elapsed = start.elapsed();
    println!("Elapsed: {:.3}ms", elapsed.as_secs_f64() * 1000.0);

    Ok(())
}
