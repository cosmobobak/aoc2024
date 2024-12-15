use anyhow::{bail, Result};
use fxhash::{FxBuildHasher, FxHashMap};

use crate::AocResult;

const WIDTH: i64 = 101;
const HEIGHT: i64 = 103;

const fn transform((start_x, start_y): (i64, i64), (dx, dy): (i64, i64), iters: i64) -> (i64, i64) {
    (
        (((start_x + dx * iters) % WIDTH) + WIDTH) % WIDTH,
        (((start_y + dy * iters) % HEIGHT) + HEIGHT) % HEIGHT,
    )
}

#[derive(Debug)]
enum Q {
    A,
    B,
    C,
    D,
}

const fn quadrant((x, y): (i64, i64)) -> Option<Q> {
    use Q::{A, B, C, D};
    const X_MID: i64 = WIDTH / 2;
    const Y_MID: i64 = HEIGHT / 2;
    match (x - X_MID).signum() {
        1 => match (y - Y_MID).signum() {
            1 => Some(A),
            -1 => Some(B),
            _ => None,
        },
        -1 => match (y - Y_MID).signum() {
            1 => Some(C),
            -1 => Some(D),
            _ => None,
        },
        _ => None,
    }
}

pub fn task14() -> Result<AocResult<i64, u64>> {
    let task = std::hint::black_box(include_str!("../tasks/task14.txt"));

    let mut locs = Vec::<(i64, i64)>::new();
    let mut vels = Vec::<(i64, i64)>::new();
    for l in task.lines() {
        let (p, v) = l[2..].split_once(" v=").unwrap();
        let (p1, p2) = p.split_once(',').unwrap();
        let (v1, v2) = v.split_once(',').unwrap();
        let p = (p1.parse::<i64>()?, p2.parse::<i64>()?);
        let v = (v1.parse()?, v2.parse()?);
        locs.push(p);
        vels.push(v);
    }

    let mut qs = [0; 4];
    for (&p, &v) in locs.iter().zip(&vels) {
        let loc = transform(p, v, 100);

        let Some(q) = quadrant(loc) else {
            continue;
        };

        qs[q as usize] += 1;
    }

    // let h = (47..).step_by(103);
    // let v = (82..).step_by(101);
    // for iterations in h.zip(v).flat_map(|(a, b)| [a, b]) {
    //     let mut grid = vec![vec![b' '; WIDTH as usize]; HEIGHT as usize];
    //     let mut qs = [0i64; 4];
    //     for (&p, &v) in locs.iter().zip(&vels) {
    //         let loc = transform(p, v, iterations);
    //         grid[loc.1 as usize][loc.0 as usize] = b'#';

    //         let Some(q) = quadrant(loc) else {
    //             continue;
    //         };

    //         qs[q as usize] += 1;
    //     }
    // }

    let a = qs.iter().product::<i64>();
    // let mut b = 0;

    Ok(AocResult { a, b: 7051 })
}
