use super::intcode::IntCode;

pub fn part1(input: &str) -> usize {
    let mut computer = IntCode::new(input);
    computer.execute();

    computer
        .output
        .into_iter()
        .skip(2)
        .step_by(3)
        .filter(|&t| t == 2)
        .count()
}

pub fn part2(input: &str) -> u32 {
    let mut computer = IntCode::new(input);
    computer.p[0] = 2;
    computer.execute();

    let (mut ballx, mut padx) = (0, 0);
    computer.output.chunks(3).for_each(|obj| {
        if obj[2] == 4 {
            ballx = obj[0];
        }
        if obj[2] == 3 {
            padx = obj[0];
        }
    });

    while !computer.halted {
        let joystick = (ballx - padx).signum();

        computer.output.clear();
        computer.execute_input(joystick);

        ballx = computer
            .output
            .chunks(3)
            .find(|obj| obj[2] == 4)
            .unwrap_or(&[0, 0, 0])[0];

        padx += joystick;
    }
    computer.output[computer.output.iter().position(|&o| o == -1).unwrap() + 2] as u32
}
