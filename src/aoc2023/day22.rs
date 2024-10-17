use itertools::Itertools;
const MAP_SIZE: usize = 350;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let bricks = build_tree(input);
    bricks
        .iter()
        .filter(|b| {
            b.supports
                .iter()
                .all(|i_other| bricks[*i_other].supported_by.len() > 1)
        })
        .count()
}
pub fn part2(input: &str) -> impl std::fmt::Display {
    let bricks = build_tree(input);
    let mut fallen = vec![false; bricks.len()];
    (0..bricks.len())
        .map(|i| {
            fallen.iter_mut().for_each(|b| *b = false);
            desintegrate(i, &bricks, &mut fallen)
        })
        .sum::<usize>()
}

fn build_tree(input: &str) -> Vec<Brick> {
    let mut bricks: Vec<_> = input.lines().map(|l| Brick::parse(l)).collect();
    let mut map = vec![(usize::MAX, 0); MAP_SIZE * MAP_SIZE];
    bricks.sort_by_key(|b| b.min.2);
    for i in 0..bricks.len() {
        let maxes = bricks[i]
            .range()
            .map(|(x, y)| map[x + MAP_SIZE * y])
            .max_set_by(|&(_, z1), &(_, z2)| z1.cmp(&z2));
        bricks[i]
            .range()
            .for_each(|(x, y)| map[x + MAP_SIZE * y] = (i, maxes[0].1 + bricks[i].height()));

        for (i_support, _) in maxes
            .into_iter()
            .filter(|&(i_support, _)| i_support != usize::MAX)
            .dedup()
        {
            bricks[i_support].supports.push(i);
            bricks[i].supported_by.push(i_support);
        }
    }
    bricks
}

fn desintegrate(i: usize, bricks: &[Brick], fallen: &mut Vec<bool>) -> usize {
    fallen[i] = true;
    bricks[i]
        .supports
        .iter()
        .map(|&i_other| {
            if bricks[i_other]
                .supported_by
                .iter()
                .all(|&i_support| fallen[i_support])
            {
                return 1 + desintegrate(i_other, bricks, fallen);
            }
            0
        })
        .sum()
}

struct Brick {
    min: (usize, usize, usize),
    max: (usize, usize, usize),
    supported_by: Vec<usize>,
    supports: Vec<usize>,
}

fn parse_coord(input: &str, delta: usize) -> (usize, usize, usize) {
    input
        .split(',')
        .map(|n| n.parse::<usize>().unwrap() + delta)
        .collect_tuple()
        .unwrap()
}
impl Brick {
    fn parse(input: &str) -> Self {
        let (left, right) = input.split_once('~').unwrap();
        Self {
            min: parse_coord(left, 0),
            max: parse_coord(right, 1),
            supported_by: vec![],
            supports: vec![],
        }
    }
    fn height(&self) -> usize {
        self.max.2 - self.min.2
    }
    fn range(&self) -> impl Iterator<Item = (usize, usize)> {
        (self.min.0..self.max.0).cartesian_product(self.min.1..self.max.1)
    }
}

#[cfg(test)]
mod day22 {

    use super::*;

    const INPUT: &'static str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), "5");
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), "7");
    }
}
