use anyhow::{bail, Context, Result};

use crate::AocResult;

pub fn task09() -> Result<AocResult<i64, i64>> {
    let task = std::hint::black_box(include_str!("../tasks/task09.txt"));

    let a = part_1(task)?;
    let b = part_2(task)?;

    Ok(AocResult { a, b })
}

struct Block {
    val: i16,
    start: u16,
    len: u16,
}

// this is a horrifically inefficient design.
#[allow(clippy::cast_possible_wrap, clippy::too_many_lines)]
fn part_2(task: &str) -> Result<i64, anyhow::Error> {
    const FREE: i16 = i16::MIN;

    let mut blocks = Vec::with_capacity(task.len() * 3 / 2);
    blocks.push(Block {
        val: FREE,
        start: 0,
        len: 0,
    });
    let mut free = false;
    let mut start = 0;
    let mut val = 0;
    for &byte in task.as_bytes() {
        let len = u16::from(byte - b'0');
        if len == 0 {
            free = !free;
            continue;
        }
        if free {
            blocks.push(Block {
                val: FREE,
                start,
                len,
            });
        } else {
            blocks.push(Block { val, start, len });
            val += 1;
        }
        start += len;
        free = !free;
    }
    // padding
    blocks.push(Block {
        val: FREE,
        start: 0,
        len: 0,
    });

    let mut try_move = blocks.len() - 1;
    loop {
        if try_move == 0 {
            break;
        }
        // println!("try_move = {try_move}");
        if blocks[try_move].val == FREE {
            // empty block
            try_move -= 1;
            continue;
        }
        let mut insert = 0;
        while insert < try_move {
            if blocks[insert].val != FREE {
                insert += 1;
                continue;
            }
            if blocks[insert].len < blocks[try_move].len {
                insert += 1;
                continue;
            }
            // move the block in:
            // 0. save the value
            let val = blocks[try_move].val;
            let len = blocks[try_move].len;
            // 1. set the source as free-space (or dead space)
            blocks[try_move].val = FREE;
            // 2. unify any free space
            match (blocks[try_move - 1].val, blocks[try_move + 1].val) {
                (FREE, FREE) => {
                    // if both free, nuke this one and the right-hand one, expanding the lhs into them.
                    blocks[try_move - 1].len += blocks[try_move].len + blocks[try_move + 1].len;
                    blocks.remove(try_move);
                    blocks.remove(try_move);
                    try_move -= 1;
                }
                (FREE, _) => {
                    blocks[try_move - 1].len += blocks[try_move].len;
                    blocks.remove(try_move);
                    try_move -= 1;
                }
                (_, FREE) => {
                    blocks[try_move].len += blocks[try_move + 1].len;
                    blocks.remove(try_move + 1);
                }
                _ => {}
            }
            // 3. set the type of the freespace as our stuff
            blocks[insert].val = val;
            // 4. either inject remaining freespace or overwrite exactly:
            if blocks[insert].len != len {
                blocks.insert(
                    insert + 1,
                    Block {
                        val: FREE,
                        start: blocks[insert].start + len,
                        len: blocks[insert].len - len,
                    },
                );
                try_move += 1;
            }
            // 5. set the first section of the freespace
            blocks[insert].len = len;

            break;
        }
        if try_move == 0 {
            break;
        }
        try_move -= 1;
    }

    // accumulate
    let mut acc = 0;
    let mut count = 0;
    for block in &blocks {
        let val = i64::from(block.len);
        if block.val >= 0 {
            let lo = count * i64::from(block.val);
            let hi = (count + (val - 1)) * i64::from(block.val);
            acc += (lo + hi) * val / 2;
        }
        count += val;
    }

    Ok(acc)
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
