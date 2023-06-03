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
    let all_r = get_rotations();

    let distances = get_distances(&scanners);

    let mut total = scanners[0].clone();
    let mut stack = vec![(0, Transformation::default())];
    let mut visited = vec![false; distances.len()];

    while let Some((index, transformation)) = stack.pop() {
        visited[index] = true;

        distances
            .iter()
            .enumerate()
            .filter(|(j, _)| !visited[*j])
            .for_each(|(j, other)| {
                let pairs: Vec<_> = distances[index]
                    .iter()
                    .flat_map(|(d, pair1)| {
                        if let Some(pair2) = other.get(&d) {
                            Some((*pair1, pair2))
                        } else {
                            None
                        }
                    })
                    .collect();
                if pairs.len() >= 12 * 11 / 2 {
                    let (pair, new_rotation, translation) = pairs
                        .iter()
                        .find_map(|pair| {
                            for r in all_r.iter() {
                                let temp = (
                                    r.iter().fold(pair.1 .0, |new_p, t| t.transform(new_p)),
                                    r.iter().fold(pair.1 .1, |new_p, t| t.transform(new_p)),
                                );
                                if let Some(translation) = get_translation(temp, pair.0) {
                                    return Some((pair, r.clone(), translation));
                                }
                            }
                            None
                        })
                        .expect("rotation not found");

                    let translation = transformation
                        .rotation
                        .iter()
                        .rev()
                        .fold(translation, |new_p, t| t.transform(new_p));

                    let mut new_t = transformation.clone();
                    new_t.rotation.extend(new_rotation.into_iter().rev());
                    let temp = (
                        rotate(&new_t.rotation, pair.1 .0),
                        rotate(&new_t.rotation, pair.1 .1),
                    );
                    new_t.rotation = Rotation::get_rotation((pair.1 .0, temp.0));
                    new_t.translation = translate(new_t.translation, translation);
                    total.extend(scanners[j].iter().map(|&p| new_t.t_and_r(p)));
                    stack.push((j, new_t));
                }
            });
    }
    total.sort_unstable();
    total.dedup();
    total.len()
}

pub fn part2(input: &str) -> u32 {
    let scanners = parse(input).finish().unwrap().1;
    let all_r = get_rotations();

    let distances = get_distances(&scanners);

    let mut stack = vec![(0, Transformation::default())];
    let mut visited = vec![false; distances.len()];
    let mut translations = Vec::with_capacity(scanners.len());

    while let Some((index, transformation)) = stack.pop() {
        visited[index] = true;

        distances
            .iter()
            .enumerate()
            .filter(|(j, _)| !visited[*j])
            .for_each(|(j, other)| {
                let pairs: Vec<_> = distances[index]
                    .iter()
                    .flat_map(|(d, pair1)| {
                        if let Some(pair2) = other.get(&d) {
                            Some((*pair1, pair2))
                        } else {
                            None
                        }
                    })
                    .collect();
                if pairs.len() >= 12 * 11 / 2 {
                    let (pair, new_rotation, translation) = pairs
                        .iter()
                        .find_map(|pair| {
                            for r in all_r.iter() {
                                let temp = (
                                    r.iter().fold(pair.1 .0, |new_p, t| t.transform(new_p)),
                                    r.iter().fold(pair.1 .1, |new_p, t| t.transform(new_p)),
                                );
                                if let Some(translation) = get_translation(temp, pair.0) {
                                    return Some((pair, r.clone(), translation));
                                }
                            }
                            None
                        })
                        .expect("rotation not found");

                    let translation = transformation
                        .rotation
                        .iter()
                        .rev()
                        .fold(translation, |new_p, t| t.transform(new_p));

                    let mut new_t = transformation.clone();
                    new_t.rotation.extend(new_rotation.into_iter().rev());
                    let temp = (
                        new_t
                            .rotation
                            .iter()
                            .rev()
                            .fold(pair.1 .0, |new_p, t| t.transform(new_p)),
                        new_t
                            .rotation
                            .iter()
                            .rev()
                            .fold(pair.1 .1, |new_p, t| t.transform(new_p)),
                    );
                    new_t.rotation = Rotation::get_rotation((pair.1 .0, temp.0));
                    new_t.translation = translate(new_t.translation, translation);
                    translations.push(new_t.translation);
                    stack.push((j, new_t));
                }
            });
    }
    translations
        .iter()
        .enumerate()
        .flat_map(|(i, s1)| {
            translations
                .iter()
                .skip(i + 1)
                .map(|s2| s1.0.abs_diff(s2.0) + s1.1.abs_diff(s2.1) + s1.2.abs_diff(s2.2))
        })
        .max()
        .unwrap()
}

fn get_distances(scanners: &[Vec<Point>]) -> Vec<HashMap<i32, (Point, Point)>> {
    scanners
        .iter()
        .map(|s| {
            s.iter()
                .enumerate()
                .flat_map(|(j, &b1)| {
                    s.iter().skip(j + 1).map(move |&b2| {
                        let p = (b2.0 - b1.0, b2.1 - b1.1, b2.2 - b1.2);
                        (p.0 * p.0 + p.1 * p.1 + p.2 * p.2, (b1, b2))
                    })
                })
                .collect::<HashMap<_, _>>()
        })
        .collect()
}

#[derive(Default, Clone)]
struct Transformation {
    translation: Point,
    rotation: Vec<Rotation>,
}

impl Transformation {
    fn t_and_r(&self, p: Point) -> Point {
        translate(self.translation, rotate(&self.rotation, p))
    }
}

#[derive(Debug, Clone)]
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
            Rotation::NegX => (-p.0, p.1, p.2),
            Rotation::NegY => (p.0, -p.1, p.2),
            Rotation::NegZ => (p.0, p.1, -p.2),
            Rotation::SwapXY => (p.1, p.0, p.2),
            Rotation::SwapXZ => (p.2, p.1, p.0),
            Rotation::SwapYZ => (p.0, p.2, p.1),
        }
    }
    fn get_rotation((p1, mut p2): (Point, Point)) -> Vec<Self> {
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
    r.iter().rev().fold(p, |new_p, t| t.transform(new_p))
}
fn get_translation(p1: (Point, Point), p2: (Point, Point)) -> Option<Point> {
    let delta1 = (p2.0 .0 - p1.0 .0, p2.0 .1 - p1.0 .1, p2.0 .2 - p1.0 .2);
    let delta2 = (p2.1 .0 - p1.1 .0, p2.1 .1 - p1.1 .1, p2.1 .2 - p1.1 .2);
    if delta1 == delta2 {
        return Some(delta1);
    }
    let delta1 = (p2.0 .0 - p1.1 .0, p2.0 .1 - p1.1 .1, p2.0 .2 - p1.1 .2);
    let delta2 = (p2.1 .0 - p1.0 .0, p2.1 .1 - p1.0 .1, p2.1 .2 - p1.0 .2);
    if delta1 == delta2 {
        return Some(delta1);
    }
    None
}
fn translate(p: Point, other: Point) -> Point {
    (p.0 + other.0, p.1 + other.1, p.2 + other.2)
}

fn get_rotations() -> Vec<Vec<Rotation>> {
    let roll = [Rotation::NegX, Rotation::SwapXZ];
    let cw = [Rotation::NegZ, Rotation::SwapYZ];
    let ccw = [Rotation::NegY, Rotation::SwapYZ];
    let mut p = (1, 2, 3);
    let p0 = p;
    let mut v = vec![vec![]];
    for r in 0..6 {
        roll.iter().for_each(|t| p = t.transform(p));
        v.push(Rotation::get_rotation((p0, p)));
        for _ in 0..3 {
            if r % 2 == 0 { &cw } else { &ccw }
                .iter()
                .for_each(|t| p = t.transform(p));
            v.push(Rotation::get_rotation((p0, p)));
        }
    }
    v
}

type Point = (i32, i32, i32);

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
