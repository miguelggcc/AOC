use std::collections::HashSet;

pub fn part1(input: &str) -> u32 {
    let nx = input.lines().next().unwrap().len();
    let mut map: Vec<_> = input
        .lines()
        .flat_map(|l| l.bytes().map(|b| b == b'#'))
        .collect();
    let ny = map.len() / nx;
    let mut ratings = HashSet::new();

    loop {
        let copy = map.clone();
        for (i, b) in map.iter_mut().enumerate() {
            let n = get_neighbours(i, nx, ny).filter(|&p| copy[p]).count();
            if n == 1 || (!*b && n == 2) {
                *b = true;
            } else {
                *b = false;
            }
        }
        //map.chunks(nx).for_each(|c|println!("{:?}",c.iter().map(|&b|if b{'#'}else{'.'}).collect::<String>()));
        let rating = map
            .iter()
            .rev()
            .fold(0, |acc, &b| (acc << 1) | u32::from(b));
        if !ratings.insert(rating) {
            return rating;
        }
    }
}

pub fn part2(_input: &str) -> String {
    String::from("Not implemented")
}

type Point = (i8, i8);
const DELTAS: [Point; 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

fn get_neighbours(pos: usize, width: usize, height: usize) -> impl Iterator<Item = usize> {
    DELTAS
        .iter()
        .filter(move |(dx, dy)| {
            let (x, y) = ((pos % width) as i8 + dx, (pos / width) as i8 + dy);
            x >= 0 && x < width as i8 && y >= 0 && y < height as i8
        })
        .map(move |(dx, dy)| (pos as i8 + dx + dy * width as i8) as usize)
}

#[cfg(test)]
mod day24 {

    use super::*;

    const INPUT: &'static str = "....#
#..#.
#..##
..#..
#....";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), 2129920);
    }
    #[test]
    #[ignore]
    fn part_2() {
        assert_eq!(part2(INPUT), "");
    }
}
