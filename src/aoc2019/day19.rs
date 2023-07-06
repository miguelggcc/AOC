use super::intcode::IntCode;

pub fn part1(input: &str) -> u32 {
    let computer = IntCode::new(input);
    let mut last = 0;

    (0..50)
        .map(|x| {
            let start = (last..50)
                .find(|&y| check((x, y), computer.clone()))
                .unwrap_or(50);
            let end = (start + 1..50)
                .find(|&y| !check((x, y), computer.clone()))
                .unwrap_or(50);
            last = start;
            (end - start) as u32
        })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let computer = IntCode::new(input);
    let (mut x, mut y) = (0, 0);

    loop {
        while !check((x, y + 99), computer.clone()) {
            x += 1;
        }
        if check((x + 99, y), computer.clone()) {
            return (10_000 * x + y) as u32;
        }
        y += 1;
    }
}

fn check((x, y): (isize, isize), mut computer: IntCode) -> bool {
    computer.execute_inputs(&[x, y]);
    computer.output.pop().unwrap() == 1
}
