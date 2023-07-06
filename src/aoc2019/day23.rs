use super::intcode::IntCode;

pub fn part1(input: &str) -> u32 {
    let ic = IntCode::new(input);
    let mut computers: [IntCode; 50] = std::array::from_fn(|i| {
        let mut computer = ic.clone();
        computer.execute_input(i as isize);
        computer
    });
    let mut packets: [Vec<isize>; 50] = std::array::from_fn(|_| vec![]);

    loop {
        for (i, computer) in computers.iter_mut().enumerate() {
            if packets[i].is_empty() {
                computer.execute_input(-1);
            } else {
                computer.execute_inputs(packets[i].drain(..).collect());
            }
            let output: Vec<_> = computer.output.drain(..).collect();
            if output.get(0) == Some(&255) {
                return output[2] as u32;
            }
            output
                .chunks(3)
                .for_each(|p| packets[p[0] as usize].extend(&p[1..]));
        }
    }
}

pub fn part2(input: &str) -> u64 {
    let ic = IntCode::new(input);
    let mut computers: [IntCode; 50] = std::array::from_fn(|i| {
        let mut computer = ic.clone();
        computer.execute_input(i as isize);
        computer
    });
    let mut packets: [Vec<isize>; 50] = std::array::from_fn(|_| vec![]);
    let mut last = vec![0, 0];
    let mut nat = vec![0, 0];

    loop {
        for (i, computer) in computers.iter_mut().enumerate() {
            let packet = if packets[i].is_empty() {
                vec![-1]
            } else {
                packets[i].drain(..).collect()
            };
            computer.execute_inputs(packet);
            let output: Vec<_> = computer.output.drain(..).collect();

            output.chunks(3).for_each(|p| {
                let (&i,xy) = p.split_first().unwrap();
                if i == 255 {
                    nat = xy.to_owned();
                } else {
                    packets[i as usize].extend_from_slice(xy)
                }
            });
        }
        if packets.iter().all(|p| p.is_empty()) {
            if nat[1] == last[1] {
                return nat[1] as u64;
            }
            packets[0] = nat.clone();
            std::mem::swap(&mut last, &mut nat);
        }
    }
}
