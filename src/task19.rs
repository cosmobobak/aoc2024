use std::cmp::Reverse;

use anyhow::{bail, Result};
use fxhash::FxHashMap;
use regex::Regex;
use smartstring::alias::String;

use crate::AocResult;

fn munch(dictionary: &[String], text: &str, cache: &mut FxHashMap<String, usize>) -> usize {
    let mut sum = 0;

    for prefix in dictionary {
        if let Some(suffix) = text.strip_prefix(prefix.as_str()) {
            if suffix.is_empty() {
                sum += 1;
            } else if let Some(&count) = cache.get(suffix) {
                sum += count;
            } else {
                let res = munch(dictionary, suffix, cache);
                cache.insert(suffix.into(), res);
                sum += res;
            }
        }
    }

    sum
}

#[allow(clippy::cast_possible_wrap)]
pub fn task19() -> Result<AocResult<i64, i64>> {
    let task = std::hint::black_box(include_str!("../tasks/task19.txt"));

    let (parts, text) = task.split_once("\n\n").unwrap();

    let dictionary = parts.split(", ").map(String::from).collect::<Vec<_>>();

    let mut a = 0;
    let mut b = 0;
    let mut cache = FxHashMap::default();
    for line in text.lines() {
        let ways = munch(&dictionary, line, &mut cache) as i64;
        a += i64::from(ways > 0);
        b += ways;
    }

    Ok(AocResult {
        a,
        b,
    })
}

