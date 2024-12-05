#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(dead_code, unused_imports)]

mod task01;
mod task02;
mod task03;
mod task04;
mod task05;

use task01::task01;
use task02::task02;
use task03::task03;
use task04::task04;
use task05::task05;

fn main() {
    task04();
}
