use anyhow::{bail, Result};
use regex::Regex;

use crate::AocResult;

pub fn task19() -> Result<AocResult<i64, i64>> {
    let task = std::hint::black_box(include_str!("../tasks/task19.txt"));

    let (parts, text) = task.split_once("\n\n").unwrap();

    let parts = parts.split(", ").collect::<Vec<_>>().join("|");
    let regex = format!("^({parts})*$");
    let regex = Regex::new(&regex).unwrap();

    let mut a = 0;
    for line in text.lines() {
        // println!("{line}: {}", regex.is_match(line));
        a += i64::from(regex.is_match(line));
    }

    let mut b = 0;

    Ok(AocResult {
        a,
        b,
    })
}
