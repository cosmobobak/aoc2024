use anyhow::{Context, Result};

use crate::AocResult;

pub fn task06() -> Result<AocResult<usize, i32>> {
    let task = std::hint::black_box(include_str!("../tasks/task06.txt"));

    let a = task.lines().count();
    let b = 0;

    Ok(AocResult { a, b })
}
