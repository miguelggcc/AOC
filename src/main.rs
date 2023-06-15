use aoc::build_new_year;

const HELP: &str = "\
AOC
Usage: aoc.exe run [OPTIONS]

OPTIONS:
  -h, --help                 Prints help information
  --new <YEAR>                Creates the necessary files for a new year
  --bin <YEAR> <DAY>          Runs part 1 and 2 of DAY of YEAR
  --bin <YEAR> <DAY> --i <N>  Runs part 1 and 2 of DAY of YEAR N times
  --bin <YEAR> -- --all       Runs part 1 and 2 of every day of YEAR
";

fn main() {
    let mut pargs = pico_args::Arguments::from_env();

    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        std::process::exit(0);
    }

    if let Ok(year) = pargs.value_from_str("--new") {
        build_new_year(year);
    } else {
        let others = pargs.finish();
        assert!(!others.is_empty(), "no arguments");
        panic!("unknown argument {:?}", others)
    }
}
