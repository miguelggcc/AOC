pub fn day2(input_path: &str) {
    //Part 1
    let input = std::fs::read_to_string(input_path).expect("Can't find input file");
    let total = input
        .lines()
        .map(|l| {
            let hands: Vec<Hand> = l.split_whitespace().map(translate).collect();
            score(hands[0], hands[1]) as u32 + hands[1] as u32
        })
        .sum::<u32>();

    println!("Total score is {}", total);

    //Part 2
    let total = input
        .lines()
        .map(|l| {
            let columns: Vec<&str> = l.split_whitespace().collect();
            let round = translate_round(columns[1]);
            score_inverse(translate(columns[0]), &round) as u32 + round as u32
        })
        .sum::<u32>();

    println!("Total score for part 2 is {}", total);
}

#[derive(PartialEq, Copy, Clone)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum Round {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

fn translate(letter: &str) -> Hand {
    match letter {
        "A" | "X" => Hand::Rock,
        "B" | "Y" => Hand::Paper,
        "C" | "Z" => Hand::Scissors,
        _ => unreachable!(),
    }
}

fn translate_round(letter: &str) -> Round {
    match letter {
        "X" => Round::Lose,
        "Y" => Round::Draw,
        "Z" => Round::Win,
        _ => unreachable!(),
    }
}

fn score(first_player: Hand, second_player: Hand) -> Round {
    if first_player == second_player {
        return Round::Draw;
    }
    match (first_player, second_player) {
        (Hand::Rock, Hand::Paper)
        | (Hand::Scissors, Hand::Rock)
        | (Hand::Paper, Hand::Scissors) => Round::Win,
        (Hand::Paper, Hand::Rock)
        | (Hand::Rock, Hand::Scissors)
        | (Hand::Scissors, Hand::Paper) => Round::Lose,
        _ => unreachable!(),
    }
}

fn score_inverse(first_player: Hand, end_round: &Round) -> Hand {
    match end_round {
        Round::Lose => match first_player {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        },
        Round::Draw => first_player,
        Round::Win => match first_player {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        },
    }
}
