use anyhow::{bail, Result};

use crate::AocResult;

fn mv(
    (mut x, mut y): (usize, usize),
    (dx, dy): (isize, isize),
    grid: &mut [Vec<u8>],
) -> (usize, usize) {
    assert_eq!(grid[x][y], b'@');
    let ox = x;
    let oy = y;
    loop {
        if grid[x][y] == b'#' {
            // hit a wall, cannot continue
            return (ox, oy);
        }
        if grid[x][y] == b'.' {
            // got an empty space, swap in
            grid[x][y] = b'O';
            // also swap in the bot
            grid[ox.wrapping_add_signed(dx)][oy.wrapping_add_signed(dy)] = b'@';
            // remove the bot from src
            grid[ox][oy] = b'.';
            return (ox.wrapping_add_signed(dx), oy.wrapping_add_signed(dy));
        }
        x = x.wrapping_add_signed(dx);
        y = y.wrapping_add_signed(dy);
    }
}

fn mv2(
    (x, mut y): (usize, usize),
    (dx, dy): (isize, isize),
    grid: &mut [Vec<u8>],
) -> (usize, usize) {
    assert_eq!(grid[x][y], b'@');
    let ox = x;
    let oy = y;
    if dx == 0 {
        loop {
            if grid[x][y] == b'#' {
                // hit a wall, cannot continue
                return (ox, oy);
            }
            if grid[x][y] == b'.' {
                if dy > 0 {
                    grid[x].copy_within(oy..y, oy + 1);
                } else {
                    grid[x].copy_within(y + 1..oy + 1, y);
                }
                grid[ox][oy] = b'.';
                return (ox, oy.wrapping_add_signed(dy));
            }
            y = y.wrapping_add_signed(dy);
        }
    } else {
        // moving up or down
        fn can_move(obstacle: (usize, usize), dx: isize, grid: &[Vec<u8>]) -> bool {
            // 1. check if there are any obstacles above us
            let above_direct = grid[obstacle.0.wrapping_add_signed(dx)][obstacle.1];
            let above_right = grid[obstacle.0.wrapping_add_signed(dx)][obstacle.1 + 1];
            let above_left = grid[obstacle.0.wrapping_add_signed(dx)][obstacle.1 - 1];

            // 2. if we're direct-blocked, fail
            if above_direct == b'#' || above_right == b'#' {
                return false;
            }

            // 3. if we have boxes above us, delegate to them
            if above_left == b'[' && !can_move((obstacle.0.wrapping_add_signed(dx), obstacle.1 - 1), dx, grid) {
                return false;
            }
            if above_direct == b'[' && !can_move((obstacle.0.wrapping_add_signed(dx), obstacle.1), dx, grid) {
                return false;
            }
            if above_right == b'[' && !can_move((obstacle.0.wrapping_add_signed(dx), obstacle.1 + 1), dx, grid) {
                return false;
            }

            // 4. we're not direct-blocked, and nothing we're pushing
            // is blocked, so we can push!
            true
        }
        fn shunt(obstacle: (usize, usize), dx: isize, grid: &mut [Vec<u8>]) {
            // 1. check if there are any obstacles above us
            let above_direct = grid[obstacle.0.wrapping_add_signed(dx)][obstacle.1] == b'[';
            let above_right = grid[obstacle.0.wrapping_add_signed(dx)][obstacle.1 + 1] == b'[';
            let above_left = grid[obstacle.0.wrapping_add_signed(dx)][obstacle.1 - 1] == b'[';
            // 2. if there are, shunt them first
            if above_left {
                shunt((obstacle.0.wrapping_add_signed(dx), obstacle.1 - 1), dx, grid);
            }
            if above_direct {
                shunt((obstacle.0.wrapping_add_signed(dx), obstacle.1), dx, grid);
            }
            if above_right {
                shunt((obstacle.0.wrapping_add_signed(dx), obstacle.1 + 1), dx, grid);
            }
            // 3. shunt ourselves
            grid[obstacle.0][obstacle.1] = b'.';
            grid[obstacle.0][obstacle.1 + 1] = b'.';
            grid[obstacle.0.wrapping_add_signed(dx)][obstacle.1] = b'[';
            grid[obstacle.0.wrapping_add_signed(dx)][obstacle.1 + 1] = b']';
        }

        // 1. check if we're direct-blocked
        if grid[x.wrapping_add_signed(dx)][y] == b'#' {
            return (ox, oy);
        }

        // 2. check if we're trying to move a box
        if grid[x.wrapping_add_signed(dx)][y] == b'[' || grid[x.wrapping_add_signed(dx)][y] == b']' {
            let left_side = if grid[x.wrapping_add_signed(dx)][y] == b'[' { y } else { y - 1 };
            if !can_move((x.wrapping_add_signed(dx), left_side), dx, grid) {
                return (ox, oy);
            }
            shunt((x.wrapping_add_signed(dx), left_side), dx, grid);
        }

        grid[x.wrapping_add_signed(dx)][y] = b'@';
        grid[ox][oy] = b'.';

        (x.wrapping_add_signed(dx), y)
    }
}

pub fn task15() -> Result<AocResult<usize, usize>> {
    let task = std::hint::black_box(include_str!("../tasks/task15.txt"));

    let (grid_text, ops) = task.split_once("\n\n").unwrap();

    let mut grid = grid_text
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let mut grid2 = grid_text
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .flat_map(|byte| match *byte {
                    b'O' => [b'[', b']'],
                    b'@' => [b'@', b'.'],
                    b => [b, b],
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let robot = grid
        .iter()
        .enumerate()
        .find_map(|(row, line)| {
            Option::zip(
                Some(row),
                line.iter()
                    .enumerate()
                    .find_map(|(col, c)| (*c == b'@').then_some(col)),
            )
        })
        .unwrap();

    let mut pos = robot;
    for op in ops.as_bytes() {
        match op {
            b'v' => pos = mv(pos, (1, 0), &mut grid),
            b'^' => pos = mv(pos, (-1, 0), &mut grid),
            b'>' => pos = mv(pos, (0, 1), &mut grid),
            b'<' => pos = mv(pos, (0, -1), &mut grid),
            _ => (),
        }
    }

    let mut pos = (robot.0, robot.1 * 2);
    for op in ops.as_bytes() {
        match op {
            b'v' => pos = mv2(pos, (1, 0), &mut grid2),
            b'^' => pos = mv2(pos, (-1, 0), &mut grid2),
            b'>' => pos = mv2(pos, (0, 1), &mut grid2),
            b'<' => pos = mv2(pos, (0, -1), &mut grid2),
            _ => continue,
        }
    }

    let mut a = 0;
    for (r, row) in grid.iter().enumerate() {
        for (c, v) in row.iter().enumerate() {
            if *v == b'O' {
                a += r * 100 + c;
            }
        }
    }

    let mut b = 0;
    for (r, row) in grid2.iter().enumerate() {
        for (c, v) in row.iter().enumerate() {
            if *v == b'[' {
                b += r * 100 + c;
            }
        }
    }

    Ok(AocResult { a, b })
}

