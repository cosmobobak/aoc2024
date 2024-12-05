use anyhow::{Context, Result};

pub fn task06() -> Result<()> {
    let start = std::time::Instant::now();
    // let task = include_str!("../tasks/task05.txt");

    // println!("Part 1: {}");
    // println!("Part 2: {}");

    let elapsed = start.elapsed();
    println!("Elapsed: {:.3}ms", elapsed.as_secs_f64() * 1000.0);

    Ok(())
}
