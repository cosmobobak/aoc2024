#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(dead_code, unused_imports, clippy::unnecessary_wraps)]

mod task01;
mod task02;
mod task03;
mod task04;
mod task05;
mod task06;
mod bucket;

use std::{fmt::Display, time::{Duration, Instant}};

use task01::task01;
use task02::task02;
use task03::task03;
use task04::task04;
use task05::task05;
use task06::task06;

struct AocResult<P1: Display, P2: Display> {
    a: P1,
    b: P2,
}

fn bench<P1: Display, P2: Display, F: Fn() -> anyhow::Result<AocResult<P1, P2>>>(f: F) -> anyhow::Result<()> {
    const ITERS: usize = 1000;

    let start = Instant::now();
    for _ in 0..ITERS {
        std::hint::black_box(f()?);
    }
    let elapsed = start.elapsed();

    println!("Elapsed: {:.3}ms", elapsed.as_secs_f64() * 1000.0);

    Ok(())
}

fn exec<P1: Display, P2: Display, F: Fn() -> anyhow::Result<AocResult<P1, P2>>>(f: F) -> anyhow::Result<()> {
    let AocResult { a, b } = f()?;
    println!("Part 1: {a}");
    println!("Part 2: {b}");
    Ok(())
}

fn main() -> anyhow::Result<()> {
    exec(task06)?;
    Ok(())
}
