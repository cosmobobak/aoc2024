use std::cmp::Ordering;

use anyhow::{Context, Result};
use arrayvec::ArrayVec;

use crate::AocResult;

pub fn task05() -> Result<AocResult<i32, i32>> {
    let task = include_str!("../tasks/task05.txt");

    let (rules, seqs) = task.split_once("\n\n").context("Failed to split")?;

    let mut forward_rules = (Vec::new(), Vec::<ArrayVec<i32, 32>>::new());

    for l in rules.lines() {
        let (a, b) = l.split_once('|').context("Failed to split")?;
        let a = a.parse::<i32>()?;
        let b = b.parse::<i32>()?;
        if let Some(idx) = forward_rules.0.iter().position(|&v| v == a) {
            forward_rules.1[idx].push(b);
        } else {
            forward_rules.0.push(a);
            forward_rules.1.push(ArrayVec::new());
            forward_rules.1.last_mut().unwrap().push(b);
        }
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
            for (&before, after_list) in forward_rules.0.iter().zip(&forward_rules.1) {
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
            forward_rules.0
                .iter()
                .zip(&forward_rules.1)
                .find_map(|(first, seconds)| if *first == *a && seconds.contains(b) {
                    Some(Ordering::Less)
                } else if *first == *b && seconds.contains(a) {
                    Some(Ordering::Greater)
                } else {
                    None
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
