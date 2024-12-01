use std::{
    collections::{BinaryHeap, HashMap},
    ops::AddAssign,
};

pub fn task01() {
    let start = std::time::Instant::now();
    let task = include_str!("../tasks/task01.txt");

    // part one structures
    let mut q1 = BinaryHeap::new();
    let mut q2 = BinaryHeap::new();

    // part two structures
    let mut xs = Vec::new();
    let mut counts = HashMap::new();

    for line in task.lines() {
        let [l, r] = line
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<Vec<i32>, _>>()
            .unwrap()[..]
        else {
            unreachable!();
        };

        q1.push(l);
        q2.push(r);

        xs.push(l);
        counts.entry(r).and_modify(|c| *c += 1).or_insert(1);
    }

    let mut sum = 0;
    while let Some((a, b)) = q1.pop().zip(q2.pop()) {
        sum += a.abs_diff(b);
    }

    println!("Part 1: {sum}");

    let sum = xs
        .iter()
        .map(|v| v * counts.get(v).unwrap_or(&0))
        .sum::<i32>();

    println!("Part 2: {sum}");

    let elapsed = start.elapsed();
    println!("Elapsed: {:.3}ms", elapsed.as_secs_f64() * 1000.0);
}
