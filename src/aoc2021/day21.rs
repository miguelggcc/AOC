pub fn part1(input: &str) -> u32 {
    let mut p0 = input
        .lines()
        .map(|l| l.split_once(": ").unwrap().1.parse::<u32>().unwrap());
    let mut players = [(0, p0.next().unwrap()), (0, p0.next().unwrap())];
    let mut dice = (1..101).cycle();
    let mut rolled = 0;
    while players.iter().all(|&p| p.0 < 1000) {
        let mut score = players[0].1 + (&mut dice).take(3).sum::<u32>();
        while score > 10 {
            score = score % 11 + score / 11;
        }
        players[0] = (players[0].0 + score, score);
        rolled += 3;
        players.rotate_left(1);
    }
    players.into_iter().map(|p| p.0).min().unwrap() * rolled
}

pub fn part2(input: &str) -> u64 {
    let p0: Vec<_> = input
        .lines()
        .map(|l| l.split_once(": ").unwrap().1.parse::<u32>().unwrap())
        .collect();
    let (u0, u1) = play((0, p0[0]), (0, p0[1]));
    u0.max(u1)
}
const DICE: [(u32, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

fn play(mut p0: (u32, u32), p1: (u32, u32)) -> (u64, u64) {
    if p1.0 >= 21 {
        return (0, 1);
    }
    let mut out = (0, 0);
    for (d, rep) in DICE.iter() {
        let score = (p0.1 + d-1) % 10 + 1;
        p0 = (p0.0 + score, score);
        let (np1, np0) = play(p1, p0);
        out = (out.0 + rep * np0, out.1 + rep * np1);
    }
    out
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
        assert_eq!(part2(INPUT), 341960390180808);
    }
}
