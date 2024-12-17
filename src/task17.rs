use anyhow::{bail, Result};
use arrayvec::ArrayVec;
use fxhash::FxHashSet;
use std::{fmt::Write, ops::ControlFlow};

use crate::AocResult;

const ADV: u8 = 0;
const BXL: u8 = 1;
const BST: u8 = 2;
const JNZ: u8 = 3;
const BXC: u8 = 4;
const OUT: u8 = 5;
const BDV: u8 = 6;
const CDV: u8 = 7;

const A: usize = 0;
const B: usize = 1;
const C: usize = 2;

pub fn task17() -> Result<AocResult<String, usize>> {
    let task = std::hint::black_box(include_str!("../tasks/task17.txt"));

    let (regs, program) = task.split_once("\n\n").unwrap();

    let regs: [usize; 3] = regs
        .lines()
        .map(|l| l.split_once(": ").unwrap().1.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
        .try_into()
        .unwrap();
    let program = program
        .trim()
        .split_once(": ")
        .unwrap()
        .1
        .split(',')
        .map(str::parse::<u8>)
        .collect::<Result<ArrayVec<_, 16>, _>>()
        .unwrap();

    let mut stdout = String::new();

    let mut loop_cache = FxHashSet::default();
    exec(regs, &program, &mut loop_cache, |val| {
        write!(stdout, "{val},").unwrap();
        ControlFlow::Continue(())
    });

    let program_length = program.len();

    let mut b = 0;
    let mut closest_so_far = 0;
    let mut loop_cache = FxHashSet::default();
    for a_init in 0.. {
        let a_init = a_init * 0o1000000000 + 0o133267275;
        let init = [a_init, 0, 0];
        let mut buf = ArrayVec::<u8, 16>::new();
        exec(init, &program, &mut loop_cache, |val| {
            if val != program[buf.len()] {
                return ControlFlow::Break(());
            }
            buf.push(val);
            if buf.len() == program_length {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        });
        if buf == program {
            b = a_init;
            break;
        }
        loop_cache.clear();
        if buf.len() > closest_so_far {
            println!("new best: Oo{a_init:o}");
            closest_so_far = buf.len();
        }
        if a_init % (2048 << 14) == 0 {
            println!("{a_init}: {closest_so_far} / {program_length}");
        }
    }

    Ok(AocResult {
        a: stdout.trim_end_matches(',').into(),
        b,
    })
}

#[allow(clippy::cast_possible_truncation)]
fn exec(mut regs: [usize; 3], program: &[u8], loop_cache: &mut FxHashSet<(usize, [usize; 3])>, mut output: impl FnMut(u8) -> ControlFlow<(), ()>) {
    let mut ip = 0;
    while ip < program.len() - 1 {
        if !loop_cache.insert((ip, regs)) {
            break;
        }
        let instruction = program[ip];
        let literal = usize::from(program[ip + 1]);
        let combo = match literal {
            4 => regs[A],
            5 => regs[B],
            6 => regs[C],
            x => x,
        };
        match instruction {
            ADV => regs[A] /= 1 << combo,
            BXL => regs[B] ^= literal,
            BST => regs[B] = combo % 8,
            JNZ => {
                if regs[A] != 0 {
                    ip = literal;
                    continue;
                }
            }
            BXC => regs[B] ^= regs[C],
            OUT => {
                if output((combo % 8) as u8) == ControlFlow::Break(()) {
                    break;
                }
            }
            BDV => regs[B] = regs[A] / (1 << combo),
            CDV => regs[C] = regs[A] / (1 << combo),
            _ => (),
        }
        ip += 2;
    }
}
