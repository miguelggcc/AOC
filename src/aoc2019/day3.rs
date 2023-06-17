pub fn part1(input: &str) -> u32 {
    let (i1, i2) = input.split_once('\n').unwrap();
    let (path1, path2) = (parse_line(i1), parse_line(i2));
    path1
        .into_iter()
        .flat_map(|(p1, _)| path2.iter().filter_map(move |(p2, _)| p1.intersection(p2)))
        .map(|(x, y)| x.unsigned_abs() + y.unsigned_abs())
        .filter(|&d| d != 0)
        .min()
        .unwrap()
}

pub fn part2(input: &str) -> u32 {
    let (i1, i2) = input.split_once('\n').unwrap();
    let (path1, path2) = (parse_line(i1), parse_line(i2));
    path1
        .into_iter()
        .flat_map(|(p1, d1)| {
            path2
                .iter()
                .filter_map(move |(p2, d2)| p1.intersection2(p2).zip(Some(d1 + d2)))
        })
        .map(|(d, extra_d)| d + extra_d)
        .filter(|&d| d != 0)
        .min()
        .unwrap()
}

type Point = (i32, i32);
#[derive(Debug)]
enum Line {
    V((Point, Point)),
    H((Point, Point)),
}

impl Line {
    fn intersection(&self, other: &Self) -> Option<Point> {
        match (self, other) {
            (Self::V((v0, v1)), Self::H((h0, h1))) | (Self::H((h0, h1)), Self::V((v0, v1)))
                if (h0.0.min(h1.0)..=h1.0.max(h0.0)).contains(&v0.0)
                    && (v0.1.min(v1.1)..=v1.1.max(v0.1)).contains(&h0.1) =>
            {
                Some((v1.0, h1.1))
            }
            _ => None,
        }
    }
    fn intersection2(&self, other: &Self) -> Option<u32> {
        match (self, other) {
            (Self::V((v0, v1)), Self::H((h0, h1))) | (Self::H((h0, h1)), Self::V((v0, v1)))
                if (h0.0.min(h1.0)..=h1.0.max(h0.0)).contains(&v0.0)
                    && (v0.1.min(v1.1)..=v1.1.max(v0.1)).contains(&h0.1) =>
            {
                Some(v1.0.abs_diff(h0.0) + h1.1.abs_diff(v0.1))
            }
            _ => None,
        }
    }
}

fn parse_line(input: &str) -> Vec<(Line, u32)> {
    input
        .split(',')
        .scan(((0, 0), 0), |acc, input| {
            let (dir, ds) = input.split_at(1);
            let d = ds.trim().parse::<i32>().unwrap();
            let p = acc.0;
            let (l, p_new) = match dir {
                "R" => (Line::H((p, (p.0 + d, p.1))), (p.0 + d, p.1)),
                "L" => (Line::H((p, (p.0 - d, p.1))), (p.0 - d, p.1)),
                "U" => (Line::V((p, (p.0, p.1 + d))), (p.0, p.1 + d)),
                "D" => (Line::V((p, (p.0, p.1 - d))), (p.0, p.1 - d)),
                e => panic!("unknown input {:?}", e),
            };
            let last_d = acc.1;
            *acc = (p_new, last_d + d.unsigned_abs());
            Some((l, last_d))
        })
        .collect()
}

#[cfg(test)]
mod day3 {

    use super::*;

    const INPUT: &'static str = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), 135);
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT), 410);
    }
}
