use anyhow::{bail, Result};
use fxhash::{FxBuildHasher, FxHashMap};

use crate::AocResult;

pub fn task12() -> Result<AocResult<i64, i64>> {
    let task = std::hint::black_box(include_str!("../tasks/task12.txt"));

    let mut a = 0;
    let mut b = 0;

    Ok(AocResult { a, b })
}
