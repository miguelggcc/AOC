use aoc::build_new_year;
use std::{process::Command, time::Instant};

fn main() {
    let mut pargs = pico_args::Arguments::from_env();

    if let Ok(year) = pargs.value_from_str("--new") {
        build_new_year(year);
    } else if let Ok(year) = pargs.value_from_str::<&str, String>("--runall") {
        let timing = (1..=25)
            .map(|d| {
                let time = Instant::now();
                Command::new("cargo")
                    .args([
                        "run",
                        "--release",
                        "--bin",
                        &year,
                        "--",
                        "--d",
                        &d.to_string(),
                    ])
                    .output()
                    .unwrap();
                time.elapsed().as_secs_f32()
            })
            .sum::<f32>();
        println!("Whole year {} took {:?}", year, timing);
    } else {
        let others = pargs.finish();
        assert!(!others.is_empty(), "no arguments");
        panic!("unknown argument {:?}", others)
    }
}
