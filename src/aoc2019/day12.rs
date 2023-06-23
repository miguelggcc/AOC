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
    for _ in 0..1000 {
        for i in 0..3{
            calculate(&mut coords, i);
        }
    }
    coords
        .into_iter()
        .map(|coord| {
            coord.iter().map(|(c, _)| c.abs()).sum::<i32>()
                * coord.iter().map(|(_, v)| v.abs()).sum::<i32>()
        })
        .sum::<i32>()
}

pub fn part2(input: &str) -> usize {
    let mut coords = parse(input).unwrap().1;
    let first = coords.clone();
    [0, 1, 2]
        .map(|i| {
            (1..)
                .find(|_| { calculate(&mut coords, i);
                coords == first
                })
                .unwrap()
        })
        .iter()
        .fold(1, |acc, &r| acc * r / gcd(acc, r))
}

fn calculate(coords: &mut Vec<[(i32,i32);3]>, i:usize){
    for spl in 0..coords.len() {
        let (left, right) = coords.split_at_mut(spl);
        let coord2 = right.first_mut().unwrap();
        for coord1 in left {
            let ((c1, v1), (c2, v2)) =
                (coord1.get_mut(i).unwrap(), coord2.get_mut(i).unwrap());
            match c1.cmp(&c2) {
                std::cmp::Ordering::Greater => {
                    *v1 -= 1;
                    *v2 += 1
                }
                std::cmp::Ordering::Less => {
                    *v1 += 1;
                    *v2 -= 1
                }
                _ => (),
            }
        }
    }
    coords
        .iter_mut()
        .for_each(|coord| coord[i].0 += coord[i].1);
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn parse(input: &str) -> IResult<&str, Vec<[(i32, i32); 3]>> {
    separated_list1(complete::line_ending, parse_coord)(input)
}

fn parse_coord(input: &str) -> IResult<&str, [(i32, i32); 3]> {
    map(
        tuple((
            preceded(tag("<x="), complete::i32),
            preceded(tag(", y="), complete::i32),
            delimited(tag(", z="), complete::i32, tag(">")),
        )),
        |(x, y, z)| [(x, 0), (y, 0), (z, 0)],
    )(input)
}

#[cfg(test)]
mod day12 {

    use super::*;

    const INPUT: &'static str = "<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>";

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT), 4686774924);
    }
}
