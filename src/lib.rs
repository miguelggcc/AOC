use std::{fs, path::Path};

#[macro_export]
macro_rules! year {
    ($year:literal) => {
        paste!{
        #[path = "../"[<aoc $year >]"/mod.rs"]
        mod [<aoc $year >];
        }

        seq!(D in 1..=25 {
        paste! {
            pub fn run(d: u32){
                match (d) {
                    #(D => {let input = std::fs::read_to_string(concat!["inputs/aoc", stringify![$year], "/input_day", stringify![D], ".txt"])
                                    .expect("can't read input file");
                                    let time = Instant::now();
                                    println!("{}",[<aoc $year >]::[<day D>]::part1(&input));
                                    println!("{:?}", time.elapsed());
                                    let time = Instant::now();
                                    println!("{}",[<aoc $year >]::[<day D>]::part2(&input));
                                    println!("{:?}", time.elapsed());})*

                    day => panic!("there is no day {day}"),
                }
            }
        }
        });
    }
}

pub fn parse_args_day() -> Result<u32, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    let day = pargs.value_from_str("--d")?;
    // It's up to the caller what to do with the remaining arguments.
    let remaining = pargs.finish();
    if !remaining.is_empty() {
        eprintln!("Warning: unused arguments left: {:?}.", remaining);
    }

    Ok(day)
}

pub fn build_new_year(year: u32) {
    let code_path_str = format!("./src/aoc{}", year);
    fs::create_dir_all(code_path_str.clone()).expect("could not create folder");
    fs::create_dir_all(format!("./inputs/aoc{}", year)).expect("could not create folder");
    for day in 1..=25 {
        let day_path = format!("{}/day{}.rs", code_path_str, day);
        let path = Path::new(&day_path);
        if !path.exists() {
            fs::write(
                path,
                "pub fn part1(_input: &str) -> u32 {
    todo!();
}

pub fn part2(_input: &str) -> u32 {
    todo!();
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &'static str = \"\";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), 0);
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT), 0);
    }
}",
            )
            .expect("could not create file");
        }

        let mod_path = format!("{}/mod.rs", code_path_str);
        let path = Path::new(&mod_path);
        if !path.exists() {
            fs::write(
                path,
                (1..=25)
                    .map(|d| format!("pub mod day{d};\n"))
                    .collect::<String>(),
            )
            .expect("could not create file")
        }
        let bin_path_str = format!("./src/bin/{}.rs", year);
        let bin_path = Path::new(&bin_path_str);
        if !bin_path.exists() {
            fs::write(
                bin_path,
                format!(
                    "use aoc::year;
use paste::paste;
use seq_macro::seq;
use std::time::Instant;

year!({});

fn main() {{
    let day = aoc::parse_args_day().unwrap_or_else(|e| panic!(\"Error {{e}}\"));
    run(day);
}}",
                    year
                ),
            )
            .expect("could not create file")
        }
    }
}
