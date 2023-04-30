use day1::day1;
use day10::day10;
use day11::day11;
use day12::day12;
use day13::day13;
use day14::day14;
use day15::day15;
use day16::day16;
use day2::day2;
use day3::day3;
use day4::day4;
use day5::day5;
use day6::day6;
use day7::day7;
use day8::day8;
use day9::day9;
mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
const DAY: u32 = 16;

fn main() {
    match DAY {
        1 => day1("inputs/input_day1.txt"),
        2 => day2("inputs/input_day2.txt"),
        3 => day3("inputs/input_day3.txt"),
        4 => day4("inputs/input_day4.txt"),
        5 => day5("inputs/input_day5.txt"),
        6 => day6("inputs/input_day6.txt"),
        7 => day7("inputs/input_day7.txt"),
        8 => day8("inputs/input_day8.txt"),
        9 => day9("inputs/input_day9.txt"),
        10 => day10("inputs/input_day10.txt"),
        11 => day11("inputs/input_day11.txt"),
        12 => day12("inputs/input_day12.txt"),
        13 => day13("inputs/input_day13.txt"),
        14 => day14("inputs/input_day14.txt"),
        15 => day15("inputs/input_day15.txt"),
        16 => day16("inputs/input_day16.txt"),
        _ => unreachable!(),
    }
}
