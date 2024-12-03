#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(dead_code, unused_imports)]

mod task01;
mod task02;
mod task03;

use task01::task01;
use task02::task02;
use task03::task03;

fn main() {
    task01();
}
