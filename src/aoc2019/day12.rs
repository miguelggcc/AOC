use nom::{
    bytes::complete::tag,
    character::complete,
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult,
};

pub fn part1(input: &str) -> i32 {
    let mut coords = parse(input).unwrap().1;
    let mut vels = vec![[0, 0, 0]; coords.len()];
    for _ in 0..1000 {
        coords.iter().enumerate().for_each(|(i, c1)| {
            coords.iter().enumerate().skip(i).for_each(|(j, c2)| {
                for vi in 0..3 {
                    match c1[vi].cmp(&c2[vi]) {
                        std::cmp::Ordering::Greater => {
                            vels[i][vi] -= 1;
                            vels[j][vi] += 1
                        }
                        std::cmp::Ordering::Less => {
                            vels[i][vi] += 1;
                            vels[j][vi] -= 1
                        }
                        _ => (),
                    }
                }
            })
        });
        coords
            .iter_mut()
            .zip(&vels)
            .for_each(|(c3, v3)| c3.iter_mut().zip(v3).for_each(|(c, v)| *c += v));
    }
    coords
        .into_iter()
        .zip(vels.into_iter())
        .map(|(c3, v3)| {
            c3.iter().map(|c| c.abs()).sum::<i32>() * v3.iter().map(|v| v.abs()).sum::<i32>()
        })
        .sum::<i32>()
}

pub fn part2(_input: &str) -> String {
    String::from("Not implemented")
}

fn parse(input: &str) -> IResult<&str, Vec<[i32; 3]>> {
    separated_list1(complete::line_ending, parse_coord)(input)
}

fn parse_coord(input: &str) -> IResult<&str, [i32; 3]> {
    map(
        tuple((
            preceded(tag("<x="), complete::i32),
            preceded(tag(", y="), complete::i32),
            delimited(tag(", z="), complete::i32, tag(">")),
        )),
        |(x, y, z)| [x, y, z],
    )(input)
}

#[cfg(test)]
mod day12 {

    use super::*;

    const INPUT: &'static str = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), 179);
    }
    #[test]
    #[ignore]
    fn part_2() {
        assert_eq!(part2(INPUT), "");
    }
}
