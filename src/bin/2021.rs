use aoc::year;
use paste::paste;
use seq_macro::seq;
use std::time::Instant;
#[path = "../aoc2021/mod.rs"]
mod aoc2021;

year!(2021);

fn main() {
    let day = aoc::parse_args_day().unwrap_or_else(|e| panic!("Error {e}"));
    run(day);
}