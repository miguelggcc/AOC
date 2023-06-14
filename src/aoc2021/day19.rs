use std::collections::HashMap;

use nom::{
    branch::permutation,
    bytes::complete::take_until,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::{preceded, terminated, tuple},
    Finish, IResult,
};

pub fn part1(input: &str) -> usize {
    let scanners = parse(input).finish().unwrap().1;
    let mut total = solve(scanners, true);
    total.sort_unstable();
    total.dedup();
    total.len()
}

pub fn part2(input: &str) -> u32 {
    let scanners = parse(input).finish().unwrap().1;
    let translations = solve(scanners, false);
    translations
        .iter()
        .enumerate()
        .flat_map(|(i, s1)| {
            translations[i + 1..]
                .iter()
                .map(|s2| s1.0.abs_diff(s2.0) + s1.1.abs_diff(s2.1) + s1.2.abs_diff(s2.2))
        })
        .max()
        .unwrap()
}

fn get_distances(scanners: &[Vec<Point>]) -> Vec<HashMap<(u32, u32, u32), Pair>> {
    scanners
        .iter()
        .map(|s| {
            s.iter()
                .enumerate()
                .flat_map(|(j, &b1)| {
                    s.iter().skip(j + 1).map(move |&b2| {
                        let p = abs_diff(b1, b2);
                        let dist = p.0 + p.1 + p.2;
                        let min = p.0.min(p.1).min(p.2);
                        let max = p.0.max(p.1).max(p.2);
                        ((dist, min, max), (b1, b2))
                    })
                })
                .collect::<HashMap<_, _>>()
        })
        .collect()
}

fn solve(scanners: Vec<Vec<Point>>, part1: bool) -> Vec<Point> {
    let all_r = get_rotations();
    let distances = get_distances(&scanners);
    let mut out = if part1 {
        scanners[0].clone()
    } else {
        Vec::with_capacity(scanners.len())
    };
    let mut stack = vec![(0, vec![], (0, 0, 0))];
    let mut visited = vec![false; distances.len()];

    while let Some((index, old_rotation, old_translation)) = stack.pop() {
        visited[index] = true;
        for (j, other) in distances.iter().enumerate().filter(|&(j, _)| !visited[j]) {
            let pairs: Vec<_> = distances[index]
                .iter()
                .filter(|(d, _)| other.contains_key(d))
                .collect();
            if pairs.len() >= 12 * 11 / 2 {
                let mut candidates = [0; 24];
                let (mut delta_rotation, delta_translation) = pairs
                    .into_iter()
                    .find_map(|(d, pair)| {
                        let other_pair = other.get(d).unwrap();
                        for (i, r) in all_r.iter().enumerate() {
                            let temp = (rotate(r, other_pair.0), rotate(r, other_pair.1));
                            let t1 = sub(pair.0, temp.0);
                            let t2 = sub(pair.1, temp.1);
                            if t1 == t2 {
                                candidates[i] += 1;
                                if candidates[i] >= 3 {
                                    return Some((r.clone(), t1));
                                }
                            }
                        }
                        None
                    })
                    .expect("rotation not found");
                let rotated_delta_translation = rotate(&old_rotation, delta_translation);
                delta_rotation.extend(old_rotation.clone());
                let temp = rotate(&delta_rotation, (1, 2, 3));
                let rotation = Rotation::get_rotation((temp, (1, 2, 3)));
                let translation = translate(old_translation, rotated_delta_translation);
                if part1 {
                    out.extend(
                        scanners[j]
                            .iter()
                            .map(|&p| translate(translation, rotate(&rotation, p))),
                    );
                } else {
                    out.push(translation);
                }
                stack.push((j, rotation.clone(), translation));
            }
        }
    }
    out
}

#[derive(Clone)]
enum Rotation {
    NegX,
    NegY,
    NegZ,
    SwapXZ,
    SwapXY,
    SwapYZ,
}

impl Rotation {
    fn transform(&self, p: Point) -> Point {
        match self {
            Self::NegX => (-p.0, p.1, p.2),
            Self::NegY => (p.0, -p.1, p.2),
            Self::NegZ => (p.0, p.1, -p.2),
            Self::SwapXY => (p.1, p.0, p.2),
            Self::SwapXZ => (p.2, p.1, p.0),
            Self::SwapYZ => (p.0, p.2, p.1),
        }
    }
    fn get_rotation((p1, mut p2): Pair) -> Vec<Self> {
        let mut t = vec![];
        if p1.0.abs() == p2.1.abs() {
            t.push(Self::SwapXY);
            p2 = Self::SwapXY.transform(p2);
        }
        if p1.0.abs() == p2.2.abs() {
            t.push(Self::SwapXZ);
            p2 = Self::SwapXZ.transform(p2);
        }
        if p1.1.abs() == p2.2.abs() {
            t.push(Self::SwapYZ);
            p2 = Self::SwapYZ.transform(p2);
        }
        if p1.0 == -p2.0 {
            t.push(Self::NegX);
        }
        if p1.1 == -p2.1 {
            t.push(Self::NegY);
        }
        if p1.2 == -p2.2 {
            t.push(Self::NegZ);
        }
        t
    }
}
fn rotate(r: &[Rotation], p: Point) -> Point {
    r.iter().fold(p, |new_p, t| t.transform(new_p))
}

fn translate(p: Point, other: Point) -> Point {
    (p.0 + other.0, p.1 + other.1, p.2 + other.2)
}
fn sub(p: Point, other: Point) -> Point {
    (p.0 - other.0, p.1 - other.1, p.2 - other.2)
}
fn abs_diff(p: Point, o: Point) -> (u32, u32, u32) {
    (o.0.abs_diff(p.0), o.1.abs_diff(p.1), o.2.abs_diff(p.2))
}

fn get_rotations() -> Vec<Vec<Rotation>> {
    let roll = [Rotation::SwapXZ, Rotation::NegX];
    let cw = [Rotation::SwapYZ, Rotation::NegZ];
    let ccw = [Rotation::SwapYZ, Rotation::NegY];
    let mut p = (1, 2, 3);
    let p0 = p;
    let mut v = vec![vec![]];
    for r in 0..6 {
        p = rotate(&roll, p);
        v.push(Rotation::get_rotation((p0, p)));
        for _ in 0..3 {
            p = rotate(if r % 2 == 0 { &cw } else { &ccw }, p);
            v.push(Rotation::get_rotation((p0, p)));
        }
    }
    v
}

type Point = (i32, i32, i32);
type Pair = (Point, Point);
fn parse(input: &str) -> IResult<&str, Vec<Vec<Point>>> {
    separated_list1(permutation((line_ending, line_ending)), parse_scanner)(input)
}
fn parse_scanner(input: &str) -> IResult<&str, Vec<Point>> {
    let (input, _) = terminated(take_until("\n"), line_ending)(input)?;
    separated_list1(
        line_ending,
        tuple((
            complete::i32,
            preceded(complete::char(','), complete::i32),
            preceded(complete::char(','), complete::i32),
        )),
    )(input)
}

#[cfg(test)]
mod day19 {

    use super::*;

    const INPUT: &'static str = "--- scanner 0 ---\n404,-588,-901\n528,-643,409\n-838,591,734\n390,-675,-793\n-537,-823,-458\n-485,-357,347\n-345,-311,381\n-661,-816,-575\n-876,649,763\n-618,-824,-621\n553,345,-567\n474,580,667\n-447,-329,318\n-584,868,-557\n544,-627,-890\n564,392,-477\n455,729,728\n-892,524,684\n-689,845,-530\n423,-701,434\n7,-33,-71\n630,319,-379\n443,580,662\n-789,900,-551\n459,-707,401

--- scanner 1 ---\n686,422,578\n605,423,415\n515,917,-361\n-336,658,858\n95,138,22\n-476,619,847\n-340,-569,-846\n567,-361,727\n-460,603,-452\n669,-402,600\n729,430,532\n-500,-761,534\n-322,571,750\n-466,-666,-811\n-429,-592,574\n-355,545,-477\n703,-491,-529\n-328,-685,520\n413,935,-424\n-391,539,-444\n586,-435,557\n-364,-763,-893\n807,-499,-711\n755,-354,-619\n553,889,-390

--- scanner 2 ---\n649,640,665\n682,-795,504\n-784,533,-524\n-644,584,-595\n-588,-843,648\n-30,6,44\n-674,560,763\n500,723,-460\n609,671,-379\n-555,-800,653\n-675,-892,-343\n697,-426,-610\n578,704,681\n493,664,-388\n-671,-858,530\n-667,343,800\n571,-461,-707\n-138,-166,112\n-889,563,-600\n646,-828,498\n640,759,510\n-630,509,768\n-681,-892,-333\n673,-379,-804\n-742,-814,-386\n577,-820,562

--- scanner 3 ---\n-589,542,597\n605,-692,669\n-500,565,-823\n-660,373,557\n-458,-679,-417\n-488,449,543\n-626,468,-788\n338,-750,-386\n528,-832,-391\n562,-778,733\n-938,-730,414\n543,643,-506\n-524,371,-870\n407,773,750\n-104,29,83\n378,-903,-323\n-778,-728,485\n426,699,580\n-438,-605,-362\n-469,-447,-387\n509,732,623\n647,635,-688\n-868,-804,481\n614,-800,639\n595,780,-596

--- scanner 4 ---\n727,592,562\n-293,-554,779\n441,611,-461\n-714,465,-776\n-743,427,-804\n-660,-479,-426\n832,-632,460\n927,-485,-438\n408,393,-506\n466,436,-512\n110,16,151\n-258,-428,682\n-393,719,612\n-211,-452,876\n808,-476,-593\n-575,615,604\n-485,667,467\n-680,325,-822\n-627,-443,-432\n872,-547,-609\n833,512,582\n807,604,487\n839,-516,451\n891,-625,532\n-652,-548,-490\n30,-46,-14";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), 79);
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT), 3621);
    }
}
