use std::{
    collections::{HashSet, VecDeque},
    iter::once,
};

pub fn part1(input: &str) -> u32 {
    let nx = input.lines().next().unwrap().len() as i32;
    let map: Vec<_> = input
        .lines()
        .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap()))
        .collect();
    let ny = map.len() as i32 / nx;

    map.iter()
        .enumerate()
        .filter(|(index, h)| {
            deltas(*index, nx, ny)
                .all(|(dx, dy)| map[(*index as i32 + dx + dy * nx) as usize] > **h)
        })
        .map(|(_, h)| h + 1)
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let nx = input.lines().next().unwrap().len() as i32;
    let map: Vec<_> = input
        .lines()
        .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap()))
        .collect();
    let ny = map.len() as i32 / nx;

    map.iter()
        .enumerate()
        .map(|(index0, h)| {
            if !deltas(index0, nx, ny)
                .all(|(dx, dy)| map[(index0 as i32 + dx + dy * nx) as usize] > *h)
            {
                return 0;
            }
            let mut visited = HashSet::new();
            visited.insert(index0);
            let mut q = VecDeque::from_iter(once(index0));
            while let Some(index) = q.pop_front() {
                let x = index as i32 % nx;
                let y = index as i32 / nx;
                for nindex in
                    deltas(index, nx, ny).map(|(dx, dy)| (x + dx + (y + dy) * nx) as usize)
                {
                    if !visited.contains(&nindex)
                        && map[nindex].saturating_sub(map[index]) >= 1
                        && map[nindex] != 9
                    {
                        visited.insert(nindex);
                        q.push_back(nindex)
                    }
                }
            }
            visited.len() as u32
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
const DELTAS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn deltas(index: usize, nx: i32, ny: i32) -> impl Iterator<Item = (i32, i32)> {
    let x = index as i32 % nx;
    let y = index as i32 / nx;
    DELTAS
        .into_iter()
        .filter(move |(dx, dy)| x + dx >= 0 && y + dy >= 0 && x + dx < nx && y + dy < ny)
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
