use std::cmp::Ordering;

use anyhow::{Context, Result};
use arrayvec::ArrayVec;

use crate::{bucket::Bucket, AocResult};

pub fn task05() -> Result<AocResult<i32, i32>> {
    let task = include_str!("../tasks/task05.txt");

    let (rules, seqs) = task.split_once("\n\n").context("Failed to split")?;

    let mut forward_rules = Bucket::<_, _, 32>::new();

    for l in rules.lines() {
        let (a, b) = l.split_once('|').context("Failed to split")?;
        let a = a.parse::<i32>()?;
        let b = b.parse::<i32>()?;
        forward_rules.push(a, b);
    }

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
            for (&before, after_list) in forward_rules.iter() {
                if let Some(b_idx) = seq.iter().position(|&v| v == before) {
                    for &after in after_list {
                        let Some(a_idx) = seq.iter().position(|&v| v == after) else {
                            continue;
                        };
                        if b_idx > a_idx {
                            invalid_seqs.push(seq);
                            return None;
                        }
                    }
                }
            }

            Some(seq[seq.len() / 2])
        })
        .sum::<i32>();

    for seq in &mut invalid_seqs {
        seq.sort_unstable_by(|a, b| {
            forward_rules
                .iter()
                .find_map(|(first, seconds)| {
                    if *first == *a && seconds.contains(b) {
                        Some(Ordering::Less)
                    } else if *first == *b && seconds.contains(a) {
                        Some(Ordering::Greater)
                    } else {
                        None
                    }
                })
                .unwrap()
        });
    }

    let invalid_seq_count = invalid_seqs
        .iter()
        .map(|seq| seq[seq.len() / 2])
        .sum::<i32>();

    Ok(AocResult {
        a: valid_seq_count,
        b: invalid_seq_count,
    })
}
