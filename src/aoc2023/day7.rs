use std::collections::HashMap;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let lut = [
        '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ];
    let hands = input
        .lines()
        .map(|l| {
            let (cards, bet, hand_value) = parse(l, &lut);
            let mut values: Vec<_> = cards.into_values().collect();
            values.sort_by_key(|&v| std::cmp::Reverse(v));

            (Type::get_type(values[0], values.get(1), hand_value), bet)
        })
        .collect::<Vec<(_, _)>>();
    get_total(hands)
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let lut = [
        'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
    ];
    let hands = input
        .lines()
        .map(|l| {
            let (mut cards, bet, hand_value) = parse(l, &lut);
            let values_joker = cards.remove(&'J').unwrap_or(0);
            let mut values: Vec<_> = cards.into_values().collect();
            values.sort_by_key(|&v| std::cmp::Reverse(v));

            (
                Type::get_type(
                    values.first().unwrap_or(&0) + values_joker,
                    values.get(1),
                    hand_value,
                ),
                bet,
            )
        })
        .collect::<Vec<(_, _)>>();
    get_total(hands)
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Type {
    HighCard(usize),
    Pair(usize),
    DoublePair(usize),
    Three(usize),
    FullHouse(usize),
    Four(usize),
    Five(usize),
}

impl Type {
    fn get_type(first: usize, second: Option<&usize>, hand_value: usize) -> Self {
        match (first, second) {
            (5, _) => Self::Five(hand_value),
            (4, _) => Self::Four(hand_value),
            (3, Some(2)) => Self::FullHouse(hand_value),
            (3, _) => Self::Three(hand_value),
            (2, Some(2)) => Self::DoublePair(hand_value),
            (2, _) => Self::Pair(hand_value),
            _ => Self::HighCard(hand_value),
        }
    }
}

fn get_total(mut hands: Vec<(Type, usize)>) -> usize {
    hands.sort_by(|(hand1, _), (hand2, _)| hand1.cmp(hand2));
    hands
        .into_iter()
        .enumerate()
        .map(|(i, (_, bet))| (i + 1) * bet)
        .sum()
}

fn parse(input: &str, lut: &[char; 13]) -> (HashMap<char, usize>, usize, usize) {
    let (hand, bet) = input.split_once(" ").unwrap();
    let mut cards: HashMap<char, usize> = HashMap::new();
    hand.chars().for_each(|c| *cards.entry(c).or_insert(0) += 1);
    let hand_value = hand.chars().fold(0, |acc, card| {
        (acc << 4) + lut.iter().position(|c| *c == card).unwrap()
    });
    (cards, bet.parse().unwrap(), hand_value)
}

#[cfg(test)]
mod day7 {

    use super::*;

    const INPUT: &'static str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), "6440");
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), "5905");
    }
}
