use std::collections::HashMap;

pub fn part1(input: &str) -> u32 {
    let mut p_iter = input
        .lines()
        .map(|l| Player::new(0, l.split_once(": ").unwrap().1.parse::<u16>().unwrap()));
    let (mut p0, mut p1) = (p_iter.next().unwrap(), p_iter.next().unwrap());
    let mut dice = (1..101).cycle();
    let mut rolled = 0;
    while p1.s < 1000 {
        let d = (&mut dice).take(3).sum::<u16>();
        rolled += 3;
        let score = (p0.p + d - 1) % 10 + 1;
        p0 = Player::new(p0.s + score, score);
        std::mem::swap(&mut p0, &mut p1);
    }
    rolled * (p0.s.min(p1.s)) as u32
}

pub fn part2(input: &str) -> u64 {
    let mut p_iter = input
        .lines()
        .map(|l: &str| Player::new(0, l.split_once(": ").unwrap().1.parse::<u16>().unwrap()));
    let (p0, p1) = (p_iter.next().unwrap(), p_iter.next().unwrap());
    let (u0, u1) = play_turn(p0, p1, &mut HashMap::new());
    u0.max(u1)
}
const DICE: [(u16, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

fn play_turn(p0: Player, p1: Player, c: &mut HashMap<u64, (u64, u64)>) -> (u64, u64) {
    if p1.s >= 21 {
        return (0, 1);
    }
    if let Some(cached_out) = c.get(&p0.get_key(&p1)) {
        return *cached_out;
    }
    let mut out = (0, 0);
    for (dice, rep) in DICE.iter() {
        let score = (p0.p + dice - 1) % 10 + 1;
        let (np1, np0) = play_turn(p1.clone(), Player::new(p0.s + score, score), c);
        out = (out.0 + rep * np0, out.1 + rep * np1);
    }
    c.insert(p0.get_key(&p1), out);
    out
}

#[derive(Clone)]
struct Player {
    p: u16,
    s: u16,
}

impl Player {
    fn new(s: u16, p: u16) -> Self {
        Self { s, p }
    }
    fn get_key(&self, other: &Self) -> u64 {
        let self32 = (self.s as u64) << 16 | self.p as u64;
        let other32 = (other.s as u64) << 16 | other.p as u64;
        other32 << 32 | self32
    }
}

#[cfg(test)]
mod day21 {

    use super::*;

    const INPUT: &'static str = "Player 1 starting position: 4
Player 2 starting position: 8";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), 739785);
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT), 444356092776315);
    }
}
