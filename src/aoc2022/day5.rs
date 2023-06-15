use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, u32},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    Finish, IResult,
};

pub fn part1(input: &str) -> String {
    let (mut crate_columns, moves) = parse(input);
    moves.iter().for_each(|m| {
        for _ in 0..m.n {
            let c = crate_columns[m.from as usize].pop().unwrap();
            crate_columns[m.to as usize].push(c);
        }
    });
    crate_columns
        .into_iter()
        .map(|c| *c.last().unwrap())
        .collect::<String>()
}

pub fn part2(input: &str) -> String {
    let (mut crate_columns, moves) = parse(input);

    moves.iter().for_each(|m| {
        let len = &crate_columns[m.from as usize].len();
        let mut cs: Vec<&str> = crate_columns[m.from as usize]
            .drain(len - m.n as usize..)
            .collect();
        crate_columns[m.to as usize].append(&mut cs);
    });
    crate_columns
        .into_iter()
        .map(|column| *column.last().unwrap())
        .collect::<String>()
}

fn parse(input: &str) -> (Vec<Vec<&str>>, Vec<Move>) {
    let mut lines = input.lines();
    let crates_lines: Vec<_> = (&mut lines)
        .map_while(|line| {
            all_consuming(parse_line)(line)
                .ok()
                .map(|(_, l)| l)
        })
        .collect();

    let mut crate_columns = vec![vec![]; crates_lines[0].len()];
    for crates in crates_lines.into_iter().rev() {
        for i in 0..crate_columns.len() {
            if let Some(crate_char) = crates[i] {
                crate_columns[i].push(crate_char);
            }
        }
    }
    assert!(lines.next().unwrap().is_empty());

    let moves = lines
        .map(|line| all_consuming(parse_move)(line).finish().unwrap().1)
        .collect();
    (crate_columns, moves)
}

fn parse_line(input: &str) -> IResult<&str, Vec<Option<&str>>> {
    separated_list1(tag(" "), parse_crate)(input)
}

fn parse_crate(input: &str) -> IResult<&str, Option<&str>> {
    let (input, c) = alt((tag("   "), delimited(tag("["), alpha1, tag("]"))))(input)?;
    let c = match c {
        "   " => None,
        value => Some(value),
    };
    Ok((input, c))
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    map(
        tuple((
            preceded(tag("move "), u32),
            preceded(tag(" from "), u32),
            preceded(tag(" to "), u32),
        )),
        |(n, f, t)| Move {
            n,
            from: f - 1,
            to: t - 1,
        },
    )(input)
}

struct Move {
    n: u32,
    from: u32,
    to: u32,
}

#[cfg(test)]
mod tests {
    use super::part1;
    use super::part2;

    #[test]
    fn part_1() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        assert_eq!(part1(input), "CMZ");
        assert_eq!(part2(input), "MCD");
    }
}
