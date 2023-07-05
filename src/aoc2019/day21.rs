use super::intcode::IntCode;

pub fn part1(input: &str) -> u64 {
    let mut ic = IntCode::new(input);
    let program = [
        "NOT B J", "NOT C T", "OR J T", "AND D T", "NOT A J", "OR T J", "WALK",
    ];
    ic.execute();

    for inst in program {
        //println!("{}", ic.get_output_ascii());
        ic.output.clear();
        ic.execute_string(inst.to_string());
    }
    ic.output.pop().unwrap() as u64
}

pub fn part2(input: &str) -> u64 {
    let mut ic = IntCode::new(input);
    let program = [
        "NOT H T", "OR C T", "AND B T", "AND A T", "NOT T J", "AND D J", "RUN",
    ];
    ic.execute();

    for inst in program {
        //println!("{}", ic.get_output_ascii());
        ic.output.clear();
        ic.execute_string(inst.to_string());
    }
    ic.output.pop().unwrap() as u64
}
