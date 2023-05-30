use nom::{
    branch::alt,
    character::complete,
    combinator::map,
    sequence::{delimited, separated_pair},
    Finish, IResult,
};

pub fn part1(input: &str) -> u32 {
    let mut lines = input.lines();
    let mut root_pair = parse_pair(lines.next().unwrap()).finish().unwrap().1;
    while let Some(line) = lines.next() {
        root_pair = El::Pair(Box::new(Pair {
            l: root_pair,
            r: parse_pair(line).finish().unwrap().1,
        }));

        loop {
            while root_pair.explode(0).0 {}
            if !root_pair.split(0) {
                break;
            }
        }
    }
    println!("{:?}", root_pair);
    0
}

pub fn part2(_input: &str) -> String {
    String::from("Not implemented")
}

#[derive(Debug)]
enum El {
    Pair(Box<Pair>),
    Value(u8),
}

impl El {
    fn explode(&mut self, depth: u8) -> (bool, u8, u8) {
        let (exploded, vl, vr, explode) = match self {
            Self::Pair(p) => match (&mut p.l, &mut p.r) {
                (Self::Pair(_), Self::Pair(_)) => {
                    let (exploded1, vl1, vr1) = p.l.explode(depth + 1);
                    p.r.add_left(vr1);
                    if !exploded1 {
                        let (exploded2, vl2, vr2) = p.r.explode(depth + 1);
                        p.l.add_right(vl2);
                        (exploded1 || exploded2, vl1, vr2, false)
                    } else {
                        (exploded1, vl1, 0, false)
                    }
                }
                (Self::Pair(_), Self::Value(vr)) => {
                    let (exploded, vl, vr2) = p.l.explode(depth + 1);
                    *vr += vr2;
                    (exploded, vl, 0, false)
                }
                (Self::Value(vl), Self::Pair(_)) => {
                    let (exploded, vl2, vr) = p.r.explode(depth + 1);
                    *vl += vl2;
                    (exploded, 0, vr, false)
                }

                (Self::Value(vl), Self::Value(vr)) => {
                    if depth >= 4 {
                        (true, *vl, *vr, true)
                    } else {
                        (false, 0, 0, false)
                    }
                }
            },
            Self::Value(_) => todo!(),
        };
        if explode {
            *self = El::Value(0);
        }
        (exploded, vl, vr)
    }

    fn add_left(&mut self, value: u8) {
        match self {
            El::Pair(p) => p.l.add_left(value),
            El::Value(v) => *v += value,
        }
    }
    fn add_right(&mut self, value: u8) {
        match self {
            El::Pair(p) => p.r.add_right(value),
            El::Value(v) => *v += value,
        }
    }
    fn split(&mut self, depth: u8) -> bool {
        match self {
            Self::Pair(p) => match (&mut p.l, &mut p.r) {
                (Self::Pair(_), Self::Pair(_)) => {
                    let explode1 = p.l.split(depth + 1);

                    if !explode1 {
                        let explode2 = p.r.split(depth + 1);
                        explode1 || explode2
                    } else {
                        explode1
                    }
                }
                (Self::Pair(_), Self::Value(vr)) => {
                    let explodel = p.l.split(depth + 1);
                    if !explodel && *vr > 9 {
                        p.r = El::Pair(Box::new(Pair {
                            l: El::Value(*vr / 2),
                            r: El::Value((*vr + 1) / 2),
                        }));
                        let mut s = String::new();
                        p.r.print(&mut s);
                        dbg!("pair,value", s);
                        return true;
                    }
                    explodel
                }
                (Self::Value(vl), Self::Pair(_)) => {
                    if *vl > 9 {
                        p.l = El::Pair(Box::new(Pair {
                            l: El::Value(*vl / 2),
                            r: El::Value((*vl + 1) / 2),
                        }));
                        let mut s = String::new();
                        p.l.print(&mut s);
                        dbg!("value,pair", s);
                        true
                    } else {
                        p.r.split(depth + 1)
                    }
                }
                (Self::Value(vl), Self::Value(vr)) => {
                    if *vl > 9 {
                        p.l = El::Pair(Box::new(Pair {
                            l: El::Value(*vl / 2),
                            r: El::Value((*vl + 1) / 2),
                        }));
                        return true;
                    }
                    if *vr > 9 {
                        p.r = El::Pair(Box::new(Pair {
                            l: El::Value(*vr / 2),
                            r: El::Value((*vr + 1) / 2),
                        }));
                        return true;
                    }
                    false
                }
            },
            Self::Value(_) => todo!(),
        }
    }

    fn print(&self, s: &mut String) {
        match self {
            El::Pair(p) => {
                s.push('[');
                p.l.print(s);
                s.push(',');
                p.r.print(s);
                s.push(']');
            }
            El::Value(v) => s.push_str(&v.to_string()),
        }
    }
}

#[derive(Debug)]
struct Pair {
    l: El,
    r: El,
}

fn parse_pair(input: &str) -> IResult<&str, El> {
    let parse_pair = map(
        delimited(
            complete::char('['),
            separated_pair(parse_pair, complete::char(','), parse_pair),
            complete::char(']'),
        ),
        |(l, r)| El::Pair(Box::new(Pair { l, r })),
    );
    let parse_integer = map(complete::u8, El::Value);

    alt((parse_pair, parse_integer))(input)
}

#[cfg(test)]
mod day18 {

    use super::*;

    const INPUT: &'static str = "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]
[7,[5,[[3,8],[1,4]]]]";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), 4140);
    }
    #[test]
    #[ignore]
    fn part_2() {
        assert_eq!(part2(INPUT), "");
    }
}
