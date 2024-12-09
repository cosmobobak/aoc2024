use anyhow::{bail, Context, Result};

use crate::AocResult;

pub fn task09() -> Result<AocResult<i64, i32>> {
    let task = std::hint::black_box(include_str!("../tasks/task09.txt"));

    let acc = part_1(task)?;

    Ok(AocResult { a: acc, b: 0 })
}

#[allow(clippy::cast_possible_wrap)]
fn part_1(task: &str) -> Result<i64, anyhow::Error> {
    let bytes = task.as_bytes();
    assert_eq!(bytes.len() % 2, 1);
    let seq = bytes.iter().copied().map(|c| c - b'0').enumerate();
    let mut rseq = bytes
        .iter()
        .rev()
        .step_by(2)
        .copied()
        .map(|c| c - b'0')
        .enumerate();
    let final_id: i64 = ((bytes.len() - 1) / 2).try_into()?;
    let (mut pulled_backward, mut curr_rev_cnt) = rseq.next().context("empty input!")?;
    let mut free = false;
    let mut count = 0;
    let mut acc = 0i64;
    'outer: for (pulled_forward, val) in seq {
        let fwd_id = (pulled_forward / 2) as i64;
        let mut bak_id = final_id - (pulled_backward) as i64;
        if fwd_id == bak_id {
            // final cleanup
            let val = val.min(curr_rev_cnt);
            let lo = count * fwd_id;
            let hi = (count + i64::from(val - 1)) * fwd_id;
            acc += (lo + hi) * i64::from(val) / 2;
            break 'outer;
        }
        if free {
            for _ in 0..val {
                // pull from the back
                if curr_rev_cnt == 0 {
                    let Some((a, b)) = rseq.next() else {
                        break 'outer;
                    };
                    (pulled_backward, curr_rev_cnt) = (a, b);
                }
                bak_id = final_id - (pulled_backward) as i64;
                if bak_id == fwd_id {
                    break 'outer;
                }
                acc += count * bak_id;
                count += 1;
                curr_rev_cnt -= 1;
            }
        } else {
            let lo = count * fwd_id;
            let hi = (count + i64::from(val - 1)) * fwd_id;
            acc += (lo + hi) * i64::from(val) / 2;
            count += i64::from(val);
        }
        free = !free;
    }

    Ok(acc)
}
