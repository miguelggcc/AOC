use super::intcode::IntCode;

pub fn part1(input: &str) -> u32 {
    let ic = IntCode::new(input);
    let mut computers: [IntCode; 50] = std::array::from_fn(|i| {
        let mut computer = ic.clone();
        computer.execute_input(i as isize);
        computer
    });

    loop {
        for i in 0..computers.len() {
            computers[i].execute_input(-1);

            let output: Vec<_> = computers[i].output.drain(..).collect();
            if output.first() == Some(&255) {
                return output[2] as u32;
            }
            output
                .chunks(3)
                .for_each(|p| computers[p[0] as usize].input.extend(&p[1..]));
        }
    }
}

pub fn part2(input: &str) -> u32 {
    let ic = IntCode::new(input);
    let mut computers: [IntCode; 50] = std::array::from_fn(|i| {
        let mut computer = ic.clone();
        computer.execute_input(i as isize);
        computer
    });
    let mut last = vec![];
    let mut nat = vec![];

    loop {
        for i in 0..computers.len() {
            computers[i].execute_input(-1);
            let output: Vec<_> = computers[i].output.drain(..).collect();

            output.chunks(3).for_each(|p| {
                let (&i, xy) = p.split_first().unwrap();
                if i == 255 {
                    nat = xy.to_vec();
                } else {
                    computers[i as usize].input.extend(xy)
                }
            });
        }
        if computers.iter().all(|c| c.input.is_empty()) {
            if nat.last() == last.last() {
                return nat[1] as u32;
            }
            computers[0].input.extend(&nat);
            std::mem::swap(&mut last, &mut nat);
        }
    }
}
