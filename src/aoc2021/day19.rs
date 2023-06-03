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

    let distances: Vec<_> = scanners
        .iter()
        .map(|s| {
            let mut v = s
                .iter()
                .enumerate()
                .flat_map(|(j, &b1)| {
                    s.iter().skip(j + 1).map(move |&b2| {
                        let p = (b2.0 - b1.0, b2.1 - b1.1, b2.2 - b1.2);
                        (p.0 * p.0 + p.1 * p.1 + p.2 * p.2, (b1, b2))
                    })
                })
                .collect::<Vec<_>>();
            v.sort_unstable_by_key(|p| p.0);
            v
        })
        .collect();

    let mut total = scanners[0].clone();
    let mut stack = vec![(
        0,
        Transformation {
            translation: (0, 0, 0),
            rotation: vec![],
        },
    )];
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
                    .filter_map(|d1| {
                        if let Some(d2) = other.iter().find(|d2| d2.0 == d1.0) {
                            return Some((d1.1, d2.1));
                        } else {
                            None
                        }
                    })
                    .collect();
                if pairs.len() >= 12 * 11 / 2 {
                    let pair = pairs
                        .iter()
                        .find(|(p, _)| {
                            (p.0 .0 != 0 || p.0 .1 != 0 || p.0 .2 != 0)
                                && (p.0 .0 != p.0 .1 && p.0 .0 != p.0 .2 && p.0 .1 != p.0 .2)
                        })
                        .expect("no unique pairs");
                    let old_pair = (
                        transformation
                            .rotation
                            .iter()
                            .rev()
                            .fold(pair.0 .0, |new_p, t| t.transform(new_p)),
                        transformation
                            .rotation
                            .iter()
                            .rev()
                            .fold(pair.0 .1, |new_p, t| t.transform(new_p)),
                    );
                    let new_rotation = all_r
                        .iter()
                        .find(|r| {
                            let temp = (
                                r.iter().fold(pair.1 .0, |new_p, t| t.transform(new_p)),
                                r.iter().fold(pair.1 .1, |new_p, t| t.transform(new_p)),
                            );
                            get_translation(temp, pair.0).is_some()
                        })
                        .unwrap()
                        .clone();
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
                    let translation = get_translation(temp, old_pair).expect("pair not found");
                    new_t.translation = translate(new_t.translation, translation);
                    total.extend(scanners[j].iter().map(|&p| {
                        translate(
                            new_t.translation,
                            new_t
                                .rotation
                                .iter()
                                .rev()
                                .fold(p, |new_p, t| t.transform(new_p)),
                        )
                    }));
                    stack.push((j, new_t));
                }
            });
    }
    total.sort_unstable();
    total.dedup();
    total.len()
}

#[derive(Debug, Clone)]
struct Transformation {
    translation: Point,
    rotation: Vec<Rotation>,
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
fn get_translation(pair1: (Point, Point), pair2: (Point, Point)) -> Option<Point> {
    let delta1 = (
        pair2.0 .0 - pair1.0 .0,
        pair2.0 .1 - pair1.0 .1,
        pair2.0 .2 - pair1.0 .2,
    );
    let delta2 = (
        pair2.1 .0 - pair1.1 .0,
        pair2.1 .1 - pair1.1 .1,
        pair2.1 .2 - pair1.1 .2,
    );
    if delta1 == delta2 {
        return Some(delta1);
    }
    let delta1 = (
        pair2.0 .0 - pair1.1 .0,
        pair2.0 .1 - pair1.1 .1,
        pair2.0 .2 - pair1.1 .2,
    );
    let delta2 = (
        pair2.1 .0 - pair1.0 .0,
        pair2.1 .1 - pair1.0 .1,
        pair2.1 .2 - pair1.0 .2,
    );
    if delta1 == delta2 {
        return Some(delta1);
    }
    None
}
fn translate(p: Point, other: Point) -> Point {
    (p.0 + other.0, p.1 + other.1, p.2 + other.2)
}

fn get_rotations() -> Vec<Vec<Rotation>> {
    let roll = [Rotation::SwapXZ, Rotation::NegX];
    let cw = [Rotation::SwapYZ, Rotation::NegZ];
    let ccw = [Rotation::SwapYZ, Rotation::NegY];
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

pub fn part2(input: &str) -> u32 {
    let scanners = parse(input).finish().unwrap().1;
    let all_r = get_rotations();

    let distances: Vec<_> = scanners
        .iter()
        .map(|s| {
            let mut v = s
                .iter()
                .enumerate()
                .flat_map(|(j, &b1)| {
                    s.iter().skip(j + 1).map(move |&b2| {
                        let p = (b2.0 - b1.0, b2.1 - b1.1, b2.2 - b1.2);
                        (p.0 * p.0 + p.1 * p.1 + p.2 * p.2, (b1, b2))
                    })
                })
                .collect::<Vec<_>>();
            v.sort_unstable_by_key(|p| p.0);
            v
        })
        .collect();

    let mut stack = vec![(
        0,
        Transformation {
            translation: (0, 0, 0),
            rotation: vec![],
        },
    )];
    let mut visited = vec![false; distances.len()];
    let mut translations = vec![];

    while let Some((index, transformation)) = stack.pop() {
        visited[index] = true;

        distances
            .iter()
            .enumerate()
            .filter(|(j, _)| !visited[*j])
            .for_each(|(j, other)| {
                let pairs: Vec<_> = distances[index]
                    .iter()
                    .filter_map(|d1| {
                        if let Some(d2) = other.iter().find(|d2| d2.0 == d1.0) {
                            return Some((d1.1, d2.1));
                        } else {
                            None
                        }
                    })
                    .collect();
                if pairs.len() >= 12 * 11 / 2 {
                    let pair = pairs
                        .iter()
                        .find(|(p, _)| {
                            (p.0 .0 != 0 || p.0 .1 != 0 || p.0 .2 != 0)
                                && (p.0 .0 != p.0 .1 && p.0 .0 != p.0 .2 && p.0 .1 != p.0 .2)
                        })
                        .expect("no unique pairs");
                    let old_pair = (
                        transformation
                            .rotation
                            .iter()
                            .rev()
                            .fold(pair.0 .0, |new_p, t| t.transform(new_p)),
                        transformation
                            .rotation
                            .iter()
                            .rev()
                            .fold(pair.0 .1, |new_p, t| t.transform(new_p)),
                    );
                    let new_rotation = all_r
                        .iter()
                        .find(|r| {
                            let temp = (
                                r.iter().fold(pair.1 .0, |new_p, t| t.transform(new_p)),
                                r.iter().fold(pair.1 .1, |new_p, t| t.transform(new_p)),
                            );
                            get_translation(temp, pair.0).is_some()
                        })
                        .unwrap()
                        .clone();
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
                    let translation = get_translation(temp, old_pair).expect("pair not found");
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

    const INPUT: &'static str = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), 79);
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT), 3621);
    }
}
