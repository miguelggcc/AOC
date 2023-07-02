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
    let mut s = String::new();
    for y in 0..50 {
        for x in 0..50 {
            let mut copy = computer.clone();
            copy.execute_inputs(vec![x, y]);
            if copy.output.pop().unwrap() == 1 {
                s.push('#');
            } else {
                s.push('.');
            }
        }
        s.push('\n');
    }

    println!("{s}");
    let mut v: Vec<_> = (0..10).map(|y| get_limits(y, &computer)).collect();

    dbg!(&v);

    for y in 10.. {
        if v[0].1 - v[9].0 >= 10 {
            return (v[0].0 * 10_000 + y) as u32;
        }
        v.rotate_left(1);
        v[9] = get_limits(y, &computer);
    }
    panic!("point not found");
}

fn get_limits(y: isize, computer: &IntCode) -> (isize, isize) {
    let start = (0..)
        .find(|&x| {
            let mut copy = computer.clone();
            copy.execute_inputs(vec![x, y]);
            copy.output.pop().unwrap() == 1
        })
        .unwrap();
    let end = (start..)
        .find(|&x| {
            let mut copy = computer.clone();
            copy.execute_inputs(vec![x, y]);
            copy.output.pop().unwrap() == 0
        })
        .unwrap();
    (start, end)
}

/*#[cfg(test)]
mod day19 {

    use super::*;

    const INPUT: &'static str = "";

    #[test]
    #[ignore]
    fn part_1() {
        assert_eq!(part1(INPUT), "");
    }
    #[test]
    #[ignore]
    fn part_2() {
        assert_eq!(part2(INPUT), "");
    }
}*/
