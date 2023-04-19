use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::alpha1,
    character::complete::u32,
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{delimited, preceded},
    Finish, IResult,
};

pub fn day5(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Can't find input file");
    //Part 1
    println!("Final top is {}", do_day5_part1(&input));
    //Part 2
    println!("Final top for part 2 is {}", do_day5_part2(&input));
}

fn do_day5_part1(input: &str) -> String {
    let mut lines = input.lines();
    let crates_lines: Vec<_> = (&mut lines)
        .map_while(|line| {
            all_consuming(parse_line)(line)
                .finish()
                .ok()
                .map(|(_input, l)| l)
        })
        .collect();

    let mut crate_columns = vec![vec![]; crates_lines[0].len()];
    for crates in crates_lines.iter().rev() {
        for i in 0..crate_columns.len() {
            if let Some(crate_char) = crates[i] {
                crate_columns[i].push(crate_char);
            }
        }
    }

    assert!(lines.next().unwrap().is_empty());

    let moves: Vec<Move> = lines
        .map(|line| all_consuming(parse_move)(line).finish().unwrap().1)
        .collect();

    moves.iter().for_each(|m| {
        for _ in 0..m.n {
            let c = crate_columns[m.from as usize].pop().unwrap();
            crate_columns[m.to as usize].push(c);
        }
    });
    crate_columns
        .iter()
        .map(|column| *column.last().unwrap())
        .collect::<Vec<&str>>()
        .join("")
}

fn parse_line(input: &str) -> IResult<&str, Vec<Option<&str>>> {
    let (input, result) = separated_list1(tag(" "), parse_crate)(input)?;
    Ok((input, result))
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
    let (input, n) = preceded(tag("move "), u32)(input)?;
    let (input, from) = preceded(tag(" from "), u32)(input)?;
    let (input, to) = preceded(tag(" to "), u32)(input)?;

    Ok((
        input,
        Move {
            n,
            from: from - 1,
            to: to - 1,
        },
    ))
}

#[derive(Debug)]
struct Move {
    n: u32,
    from: u32,
    to: u32,
}

fn do_day5_part2(input: &str) -> String {
    let mut lines = input.lines();
    let crates_lines: Vec<_> = (&mut lines)
        .map_while(|line| {
            all_consuming(parse_line)(line)
                .finish()
                .ok()
                .map(|(_input, l)| l)
        })
        .collect();

    let mut crate_columns = vec![vec![]; crates_lines[0].len()];
    for crates in crates_lines.iter().rev() {
        for i in 0..crate_columns.len() {
            if let Some(crate_char) = crates[i] {
                crate_columns[i].push(crate_char);
            }
        }
    }

    assert!(lines.next().unwrap().is_empty());

    let moves: Vec<Move> = lines
        .map(|line| all_consuming(parse_move)(line).finish().unwrap().1)
        .collect();

    moves.iter().for_each(|m| {
        let len = &crate_columns[m.from as usize].len();
        let cs: Vec<&str> = crate_columns[m.from as usize]
            .drain(len - m.n as usize..)
            .collect();
        crate_columns[m.to as usize].extend_from_slice(&cs);
    });
    crate_columns
        .iter()
        .map(|column| *column.last().unwrap())
        .collect::<Vec<&str>>()
        .join("")
}

/*#[cfg(test)]
mod tests {
    use super::do_day5_part1;
    use super::do_day5_part2;

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

        assert_eq!(do_day5_part1(input), "CMZ");
        assert_eq!(do_day5_part2(input), "MCD");
    }
}*/
