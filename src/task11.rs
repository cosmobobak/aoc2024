use anyhow::{bail, Result};
use fxhash::{FxBuildHasher, FxHashMap};

use crate::AocResult;

fn split_count(
    iters_remaining: usize,
    value: u64,
    cache: &mut FxHashMap<(usize, u64), u64>,
) -> u64 {
    if iters_remaining == 0 {
        return 1;
    }

    let key = (iters_remaining, value);

    if let Some(res) = cache.get(&key) {
        return *res;
    }

    let digits = if value == 0 { 1 } else { value.ilog10() + 1 };
    let res = if value == 0 {
        split_count(iters_remaining - 1, 1, cache)
    } else if digits % 2 == 0 {
        let split = 10u64.pow(digits / 2);
        let hi = split_count(iters_remaining - 1, value / split, cache);
        let lo = split_count(iters_remaining - 1, value % split, cache);
        lo + hi
    } else {
        split_count(iters_remaining - 1, value * 2024, cache)
    };

    cache.insert(key, res);

    res
}

pub fn task11() -> Result<AocResult<u64, u64>> {
    let task = std::hint::black_box(include_str!("../tasks/task11.txt"));

    let mut a = 0;
    let mut b = 0;
    let mut cache = FxHashMap::with_capacity_and_hasher(300_000, FxBuildHasher::default());
    for num in task.split_whitespace() {
        let num = num.parse::<u64>()?;
        a += split_count(25, num, &mut cache);
        b += split_count(75, num, &mut cache);
    }

    Ok(AocResult { a, b })
}
