use day1::day1;
use day2::day2;
use day3::day3;
use day4::day4;
use day5::day5;
use day6::day6;
use day7::day7;
use day8::day8;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

const DAY: u32 = 8;

fn main() {
    match DAY {
        1 => day1("input_day1.txt"),
        2 => day2("input_day2.txt"),
        3 => day3("input_day3.txt"),
        4 => day4("input_day4.txt"),
        5 => day5("input_day5.txt"),
        6 => day6("input_day6.txt"),
        7 => day7("input_day7.txt"),
        8 => day8("input_day8.txt"),
        _ => unreachable!(),
    }
}
