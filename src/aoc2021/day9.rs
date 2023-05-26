use std::collections::VecDeque;

pub fn part1(input: &str) -> u32 {
    let (map, nx, ny) = parse(input);
    map.iter()
        .enumerate()
        .filter(|(index, h)| neighbours(*index, nx, ny).all(|nindex| map[nindex] > **h))
        .map(|(_, h)| h + 1)
        .sum()
}

#[allow(clippy::needless_collect)] //False positive
pub fn part2(input: &str) -> u32 {
    let (mut map, nx, ny) = parse(input);
    let lows: Vec<_> = (0..map.len())
        .filter(|index| neighbours(*index, nx, ny).all(|nindex| map[nindex] > map[*index]))
        .collect();

    lows.into_iter()
        .map(|index0| {
            let mut basin = 1;
            let mut q = VecDeque::from([index0]);
            map[index0] = 9;
            while let Some(index) = q.pop_front() {
                for nindex in neighbours(index, nx, ny) {
                    if map[nindex] != 9 {
                        basin += 1;
                        map[nindex] = 9;
                        q.push_back(nindex);
                    }
                }
            }
            basin
        })
        .fold([0; 3], |mut max, basin| {
            if basin > max[0] {
                max.rotate_right(1);
                max[0] = basin;
            } else if basin > max[1] {
                max[2] = max[1];
                max[1] = basin;
            } else {
                max[2] = max[2].max(basin);
            }
            max
        })
        .iter()
        .product()
}

fn parse(input: &str) -> (Vec<u32>, i32, i32) {
    let nx = input.lines().next().unwrap().len() as i32;
    let map: Vec<_> = input
        .lines()
        .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap()))
        .collect();
    let ny = map.len() as i32 / nx;
    (map, nx, ny)
}

const DELTAS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn neighbours(index: usize, nx: i32, ny: i32) -> impl Iterator<Item = usize> {
    let x = index as i32 % nx;
    let y = index as i32 / nx;
    DELTAS
        .into_iter()
        .filter(move |(dx, dy)| x + dx >= 0 && y + dy >= 0 && x + dx < nx && y + dy < ny)
        .map(move |(dx, dy)| (index as i32 + dx + dy * nx) as usize)
}

#[cfg(test)]
mod day9 {

    use super::*;

    const INPUT: &'static str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), 15);
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT), 1134);
    }
}
