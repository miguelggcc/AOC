use std::{fmt::Write, fs, path::Path};

#[macro_export]
macro_rules! year {
    ($year:literal) => {

        seq!(D in 1..=25 {
        paste! {
            pub fn run(d: u32){
                match (d) {
                    #(D => {let input = std::fs::read_to_string(
                        concat!["inputs/aoc", stringify![$year], "/input_day", stringify![D], ".txt"]
                    ).expect("can't read input file");
                                    println!("Day {}",D);
                                    let time = Instant::now();
                                    let p1 = [<aoc $year >]::[<day D>]::part1(&input);
                                    println!("{}\n in {:?}", p1, time.elapsed());
                                    let time = Instant::now();
                                    let p2 = [<aoc $year >]::[<day D>]::part2(&input);
                                    println!("{}\n in {:?}", p2, time.elapsed());
                            })*

                    day => panic!("there is no day {day}"),
                }
            }
            pub fn run_all(){
                #(let input~D = std::fs::read_to_string(
                    concat!["inputs/aoc", stringify![$year], "/input_day", stringify![D], ".txt"]
                ).expect("can't read input file");)*
            let time = Instant::now();
            #([<aoc $year >]::[<day D>]::part1(&input~D);
            [<aoc $year >]::[<day D>]::part2(&input~D);)*
            println!("Whole year {} took {:?}", stringify![$year], time.elapsed());
            }
        }
        });
        fn main() {
            if let Some((day, iterations)) = aoc::parse_args_day().unwrap_or_else(|e| panic!("Error {e}")){
            for _ in 0..iterations {
                run(day);
            }
        } else{
            run_all();
        }
        }
    }
}

pub fn parse_args_day() -> Result<Option<(u32, usize)>, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    if pargs.contains("--all") {
        return Ok(None);
    }
    let day = pargs.free_from_str()?;
    let iterations = pargs.value_from_str("--i").unwrap_or(1);
    let remaining = pargs.finish();
    if !remaining.is_empty() {
        eprintln!("Warning: unused arguments left: {:?}.", remaining);
    }

    Ok(Some((day, iterations)))
}

pub fn build_new_year(year: u32) {
    let code_path_str = format!("./src/aoc{}", year);
    fs::create_dir_all(code_path_str.clone()).expect("could not create folder");
    fs::create_dir_all(format!("./inputs/aoc{}", year)).expect("could not create folder");

    for day in 1..=25 {
        let input_path_str = format!("./inputs/aoc{year}/input_day{day}.txt");
        let input_path = Path::new(&input_path_str);
        if !input_path.exists() {
            fs::File::create(input_path)
                .expect(&("could not create input file ".to_owned() + &input_path_str));
        }
        let day_path_str = format!("{}/day{}.rs", code_path_str, day);
        let path = Path::new(&day_path_str);
        if !path.exists() {
            fs::write(
                path,
                format!(
                    "pub fn part1(_input: &str) -> impl std::fmt::Display {{
    \"Not implemented\"
}}

pub fn part2(_input: &str) -> impl std::fmt::Display {{
    \"Not implemented\"
}}

#[cfg(test)]
mod day{day} {{

    use super::*;

    const INPUT: &'static str = \"\";

    #[test]
    #[ignore]
    fn part_1() {{
        assert_eq!(part1(INPUT).to_string(), \"\");
    }}
    #[test]
    #[ignore]
    fn part_2() {{
        assert_eq!(part2(INPUT).to_string(), \"\");
    }}
}}"
                ),
            )
            .expect(&("could not create file ".to_owned() + &day_path_str));
        }

        let mod_path_str = format!("{}/mod.rs", code_path_str);
        let path = Path::new(&mod_path_str);
        if !path.exists() {
            fs::write(
                path,
                (1..=25).fold(String::new(), |mut output, d| {
                    let _ = writeln!(output, "pub mod day{d};");
                    output
                }),
            )
            .expect(&("could not create file ".to_owned() + &mod_path_str))
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
#[path = \"../aoc{year}/mod.rs\"]
mod aoc{year};

year!({year});
"
                ),
            )
            .expect(&("could not create file ".to_owned() + &bin_path_str))
        }
    }
}
