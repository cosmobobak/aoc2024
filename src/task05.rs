use std::cmp::Ordering;

use anyhow::{Context, Result};

pub fn task05() -> Result<()> {
    let start = std::time::Instant::now();
    let task = include_str!("../tasks/task05.txt");

    let (rules, seqs) = task.split_once("\n\n").context("Failed to split")?;

    let rules = rules
        .lines()
        .map(|l| {
            let (a, b) = l.split_once('|').context("Failed to split")?;
            let a = a.parse::<i32>()?;
            let b = b.parse::<i32>()?;
            Ok((a, b))
        })
        .collect::<Result<Vec<_>>>()?;

    let seqs = seqs
        .lines()
        .map(|seq| {
            let s = seq
                .split(',')
                .map(str::parse)
                .collect::<Result<Vec<i32>, _>>()?;
            Ok(s)
        })
        .collect::<Result<Vec<_>>>()?;

    let mut invalid_seqs = Vec::new();
    let valid_seq_count = seqs
        .into_iter()
        .filter_map(|seq| {
            for (before, after) in &rules {
                if let Some(b_idx) = seq.iter().position(|v| v == before) {
                    let Some(a_idx) = seq.iter().position(|v| v == after) else {
                        continue;
                    };
                    if b_idx > a_idx {
                        invalid_seqs.push(seq);
                        return None;
                    }
                }
            }

            Some(seq[seq.len() / 2])
        })
        .sum::<i32>();

    for seq in &mut invalid_seqs {
        seq.sort_unstable_by(|&a, &b| {
            rules
                .iter()
                .find_map(|&rule| match rule {
                    p if p == (a, b) => Some(Ordering::Less),
                    p if p == (b, a) => Some(Ordering::Greater),
                    _ => None,
                })
                .unwrap()
        });
    }

    let invalid_seq_count = invalid_seqs
        .iter()
        .map(|seq| seq[seq.len() / 2])
        .sum::<i32>();

    println!("Part 1: {valid_seq_count}");
    println!("Part 2: {invalid_seq_count}");

    let elapsed = start.elapsed();
    println!("Elapsed: {:.3}ms", elapsed.as_secs_f64() * 1000.0);

    Ok(())
}
