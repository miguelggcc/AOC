use super::intcode::IntCode;
use itertools::*;

pub fn part1(input: &str) -> usize {
    let computer = IntCode::new(input);
    (0..50)
        .cartesian_product(0..50)
        .filter(|&(x, y)| {
            let mut copy = computer.clone();
            copy.execute_inputs(vec![x, y]);
            copy.output.pop().unwrap() == 1
        })
        .count()
}

pub fn part2(input: &str) -> u32 {
    let computer = IntCode::new(input);

    let mut v: Vec<_> = (0..100).map(|x| get_limits(x, 0, &computer)).collect();

    for x in 100.. {
        if v[0].1 - v[99].0 >= 100 {
            return (v[99].0 + (x - 100) * 10_000) as u32;
        }
        v.rotate_left(1);
        v[99] = get_limits(x, v[98].0, &computer);
    }
    panic!("point not found");
}

fn get_limits(x: isize, last: isize, computer: &IntCode) -> (isize, isize) {
    let start = (last..)
        .find(|&y| {
            let mut copy = computer.clone();
            copy.execute_inputs(vec![x, y]);
            copy.output.pop().unwrap() == 1
        })
        .unwrap();
    let end = (start..)
        .find(|&y| {
            let mut copy = computer.clone();
            copy.execute_inputs(vec![x, y]);
            copy.output.pop().unwrap() == 0
        })
        .unwrap();
    (start, end)
}
