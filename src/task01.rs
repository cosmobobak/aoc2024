
pub fn task01() {
    let start = std::time::Instant::now();
    let task = include_str!("../tasks/task01.txt");

    let mut ls = Vec::new();
    let mut rs = Vec::new();
    let mut keys = Vec::new();
    let mut vals = Vec::new();

    for line in task.lines() {
        let [l, r] = line
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<Vec<i32>, _>>()
            .unwrap()[..]
        else {
            unreachable!();
        };

        ls.push(l);
        rs.push(r);

        if let Some(i) = keys.iter().position(|&k| k == r) {
            vals[i] += 1;
        } else {
            keys.push(r);
            vals.push(1);
        }
    }

    ls.sort_unstable();
    rs.sort_unstable();

    let diff_sum = ls
        .iter()
        .zip(&rs)
        .map(|(&a, &b)| i32::abs_diff(a, b))
        .sum::<u32>();

    println!("Part 1: {diff_sum}");

    let count_sum = ls
        .iter()
        .map(|&v| v * keys.iter().position(|&k| k == v).map_or(0, |i| vals[i]))
        .sum::<i32>();

    println!("Part 2: {count_sum}");

    let elapsed = start.elapsed();
    println!("Elapsed: {:.3}ms", elapsed.as_secs_f64() * 1000.0);
}
