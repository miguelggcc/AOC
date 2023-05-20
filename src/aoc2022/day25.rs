pub fn part1(input: &str) -> String {
    let mut decimal = input
        .lines()
        .flat_map(|l| {
            l.char_indices().map(|(i, c)| {
                5i64.pow((l.len() - 1 - i) as u32)
                    * match c {
                        '0' | '1' | '2' => c.to_digit(3).unwrap() as i64,
                        '-' => -1,
                        '=' => -2,
                        e => panic!("unexpected char {e}"),
                    }
            })
        })
        .sum::<i64>();

    let mut out = vec![];
    while decimal > 0 {
        let digit = decimal % 5;
        out.push(CHARS[digit as usize]);
        decimal = decimal / 5 + digit / 3; // digit / 3 is the carry over
    }
    out.iter().rev().collect()
}

pub fn part2(_: &str) -> String {
    String::from("No part 2")
}

const CHARS: [char; 5] = ['0', '1', '2', '=', '-'];

#[cfg(test)]
mod tests {

    use super::part1;

    const INPUT: &'static str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), "2=-1=0");
    }
}
