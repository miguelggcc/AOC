use itertools::Itertools;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let min = 200000000000000.0;
    let max = 400000000000000.0;
    let hailstones = parse(input);
    hailstones
        .iter()
        .enumerate()
        .flat_map(|(i, &(p1, v1))| {
            hailstones[i..].iter().filter(move |&(p2, mut v2)| {
                v2 = (v2.0 * -1, v2.1 * -1, 0);
                let det = det_2d(v1, v2);
                if det == 0 {
                    return false;
                }
                let c = (p2.0 - p1.0, p2.1 - p1.1, 0);
                let (a, b) = (
                    det_2d(c, v2) as f64 / det as f64,
                    det_2d(v1, c) as f64 / det as f64,
                );
                if a.is_sign_negative() || b.is_sign_negative() {
                    return false;
                }
                let (x_int, y_int) = (p1.0 as f64 + a * v1.0 as f64, p1.1 as f64 + a * v1.1 as f64);
                x_int >= min && x_int <= max && y_int >= min && y_int <= max
            })
        })
        .count()
}

fn det_2d(a: Vec3, b: Vec3) -> i64 {
    a.0 * b.1 - a.1 * b.0
}

pub fn part2(_input: &str) -> impl std::fmt::Display {
    "Not implemented"
}
type Vec3 = (i64, i64, i64);
fn parse(input: &str) -> Vec<(Vec3, Vec3)> {
    let to_vec = |s: &str| {
        s.split(',')
            .map(|n| n.trim().parse().unwrap())
            .collect_tuple()
            .unwrap()
    };
    input
        .lines()
        .map(|l| {
            let (p, v) = l.split_once('@').unwrap();
            (to_vec(p), to_vec(v))
        })
        .collect()
}
#[cfg(test)]
mod day24 {

    use super::*;

    const INPUT: &'static str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), "2");
    }
    #[test]
    #[ignore]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), "");
    }
}
