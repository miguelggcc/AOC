use day1::day1;
use day10::day10;
use day11::day11;
use day12::day12;
use day13::day13;
use day14::day14;
use day15::day15;
use day16::day16;
use day17::day17;
use day18::day18;
use day19::day19;
use day2::day2;
use day20::day20;
use day21::day21;
use day22::day22;
use day23::day23;
use day24::day24;
use day25::day25;
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
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
const DAY: u32 = 25;

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
        17 => day17("inputs/input_day17.txt"),
        18 => day18("inputs/input_day18.txt"),
        19 => day19("inputs/input_day19.txt"),
        20 => day20("inputs/input_day20.txt"),
        21 => day21("inputs/input_day21.txt"),
        22 => day22("inputs/input_day22.txt"),
        23 => day23("inputs/input_day23.txt"),
        24 => day24("inputs/input_day24.txt"),
        25 => day25("inputs/input_day25.txt"),

        _ => unreachable!(),
    }
}
