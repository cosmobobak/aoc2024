use anyhow::{bail, Result};
use fxhash::FxHashMap;

use crate::AocResult;

fn split_count(iters_remaining: usize, value: u64, cache: &mut FxHashMap<(usize, u64), u64>) -> u64 {
    if let Some(res) = cache.get(&(iters_remaining, value)) {
        return *res;
    }

    if iters_remaining == 0 {
        return 1;
    }

    if value == 0 {
        let res = split_count(iters_remaining - 1, 1, cache);
        cache.insert((iters_remaining, value), res);
        return res;
    }

    let digits = value.ilog10() + 1;
    if digits % 2 == 0 {
        let split = 10u64.pow(digits / 2);
        let hi = split_count(iters_remaining - 1, value / split, cache);
        let lo = split_count(iters_remaining - 1, value % split, cache);
        let res = lo + hi;
        cache.insert((iters_remaining, value), res);
        return res;
    }

    let res = split_count(iters_remaining - 1, value * 2024, cache);
    cache.insert((iters_remaining, value), res);
    res
}

pub fn task11() -> Result<AocResult<u64, i32>> {
    let task = std::hint::black_box(include_str!("../tasks/task11.txt"));

    let mut a = 0;
    let mut cache = FxHashMap::default();
    for num in task.split_whitespace() {
        let num = num.parse::<u64>()?;
        a += split_count(75, num, &mut cache);
    }

    Ok(AocResult {
        a,
        b: 0,
    })
}
