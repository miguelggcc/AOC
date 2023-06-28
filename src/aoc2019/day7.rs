use super::intcode::IntCode;
use itertools::*;

pub fn part1(input: &str) -> u32 {
    let intcode = IntCode::new(input);
    (0..5)
        .permutations(5)
        .map(|n| {
            n.into_iter().fold(0, |acc, d| {
                let mut copy = intcode.clone();
                copy.execute_inputs(vec![d, acc]);
                copy.output.pop().unwrap()
            }) as u32
        })
        .max()
        .unwrap()
}

pub fn part2(input: &str) -> u32 {
    let ic = IntCode::new(input);
    let ics = [ic.clone(), ic.clone(), ic.clone(), ic.clone(), ic];
    (5..10)
        .permutations(5)
        .map(|vn| {
            let mut out = 0;
            let mut amps = ics.clone();
            amps.iter_mut()
                .zip(&vn)
                .for_each(|(a, &n)| a.execute_input(n));
            while !amps[4].halted {
                for amp in amps.iter_mut() {
                    amp.execute_input(out);
                    out = amp.output.pop().unwrap();
                }
            }
            out as u32
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod day7 {

    use super::*;

    #[test]
    fn part_1() {
        let input = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
        assert_eq!(part1(input), 54321);
    }
    #[test]
    fn part_2() {
        let input =
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
        assert_eq!(part2(input), 139629729);
    }
}
