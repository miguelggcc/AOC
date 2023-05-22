use aoc::year;
use paste::paste;
use seq_macro::seq;
use std::time::Instant;
#[path = "../aoc2022/mod.rs"]
mod aoc2022;
year!(2022);

fn main() {
    let (day, iterations) = aoc::parse_args_day().unwrap_or_else(|e| panic!("Error {e}"));
    for _ in 0..iterations {
        run(day);
    }
}
