#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(
    dead_code,
    unused_imports,
    clippy::unnecessary_wraps,
    clippy::range_plus_one
)]

mod bucket;
mod task01;
mod task02;
mod task03;
mod task04;
mod task05;
mod task06;
mod task07;
mod task08;
mod task09;
mod task10;
mod task11;
mod task12;
mod task13;
mod task14;
mod task15;
mod task16;
mod task17;
mod task18;
mod task19;

use std::{
    fmt::Display,
    time::{Duration, Instant},
};

use task01::task01;
use task02::task02;
use task03::task03;
use task04::task04;
use task05::task05;
use task06::task06;
use task07::task07;
use task08::task08;
use task09::task09;
use task10::task10;
use task11::task11;
use task12::task12;
use task13::task13;
use task14::task14;
use task15::task15;
use task16::task16;
use task17::task17;
use task18::task18;
use task19::task19;

struct AocResult<P1: Display, P2: Display> {
    a: P1,
    b: P2,
}

#[allow(clippy::cast_precision_loss)]
fn bench<P1: Display, P2: Display, F: Fn() -> anyhow::Result<AocResult<P1, P2>>>(
    f: F,
) -> anyhow::Result<()> {
    const ITERS: usize = 2000;

    let start = Instant::now();
    for _ in 0..ITERS {
        std::hint::black_box(f()?);
    }
    let elapsed = start.elapsed();

    println!("Elapsed: {:.3}ms", elapsed.as_secs_f64() * 1000.0);
    println!("Mean execution time: {:.3}ms", elapsed.as_secs_f64() * 1000.0 / ITERS as f64);

    Ok(())
}

fn exec<P1: Display, P2: Display, F: Fn() -> anyhow::Result<AocResult<P1, P2>>>(
    f: F,
) -> anyhow::Result<()> {
    let start = Instant::now();
    let AocResult { a, b } = f()?;
    let elapsed = start.elapsed();
    println!("Part 1: {a}");
    println!("Part 2: {b}");
    println!("Elapsed: {:.3}ms", elapsed.as_secs_f64() * 1000.0);
    Ok(())
}

fn main() -> anyhow::Result<()> {
    exec(task19)?;
    Ok(())
}
