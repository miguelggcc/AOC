use std::collections::{hash_map::Entry, HashMap, HashSet, VecDeque};

use super::intcode::IntCode;

pub fn part1(input: &str) -> u32 {
    let ic = IntCode::new(input);
    let mut q = VecDeque::from([((0, 0), ic, 0)]);
    let mut map = HashSet::new();
    while let Some((p, computer, d)) = q.pop_front() {
        for (i, m) in MOVES.iter().enumerate() {
            let new_p = (p.0 + m.0, p.1 + m.1);
            if !map.contains(&new_p) {
                let mut copy = computer.clone();
                copy.execute_input(i as isize + 1);
                match copy.output.pop().unwrap() {
                    0 => {
                        map.insert(new_p);
                    }
                    1 => {
                        map.insert(new_p);
                        q.push_back((new_p, copy, d + 1));
                    }
                    2 => return d + 1,
                    e => panic!("unkown command {e}"),
                }
            }
        }
    }
    panic!("oxygen system not found")
}

pub fn part2(input: &str) -> u32 {
    let ic = IntCode::new(input);
    let mut q = VecDeque::from([((0, 0), ic)]);
    let mut map = HashMap::new();
    let mut oxygen_p = (0, 0);
    while let Some((p, computer)) = q.pop_front() {
        for (i, m) in MOVES.iter().enumerate() {
            let new_p = (p.0 + m.0, p.1 + m.1);
            if let Entry::Vacant(e) = map.entry(new_p) {
                let mut copy = computer.clone();
                copy.execute_input(i as isize + 1);
                match copy.output.pop().unwrap() {
                    0 => {
                        e.insert(0);
                    }
                    1 => {
                        e.insert(1);
                        q.push_back((new_p, copy));
                    }
                    2 => {
                        oxygen_p = new_p;
                    }
                    e => panic!("unkown command {e}"),
                }
            }
        }
    }

    assert!(oxygen_p != (0, 0));

    let mut q = VecDeque::from([(oxygen_p, 0)]);
    let mut time_max = 0;
    while let Some((p, time)) = q.pop_front() {
        for m in MOVES {
            let new_p = (p.0 + m.0, p.1 + m.1);
            if map.get(&new_p) == Some(&1) {
                *map.get_mut(&new_p).unwrap() = 0;
                q.push_back((new_p, time + 1));
                time_max = time + 1;
            }
        }
    }
    time_max
}

const MOVES: [(i32, i32); 4] = [(0, 1), (0, -1), (-1, 0), (1, 0)];
