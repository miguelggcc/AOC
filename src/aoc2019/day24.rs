use std::collections::{HashMap, HashSet};

pub fn part1(input: &str) -> u32 {
    let mut map = parse(input);
    let mut ratings = HashSet::new();

    loop {
        let copy = map.clone();
        for (i, b) in map.iter_mut().enumerate() {
            let n = get_neighbours(i).filter(|&p| copy[p]).count();
            if n == 1 || (!*b && n == 2) {
                *b = true;
            } else {
                *b = false;
            }
        }
        let rating = map
            .iter()
            .rev()
            .fold(0, |acc, &b| (acc << 1) | u32::from(b));
        if !ratings.insert(rating) {
            return rating;
        }
    }
}

pub fn part2(input: &str) -> usize {
    let map = parse(input);
    let mut maps = HashMap::new();
    maps.insert(0i16, map);
    let mut copy = maps.clone();

    for _ in 0..200 {
        update_bugs(&mut maps, &mut copy);
    }
    maps.into_iter()
        .map(|(_, m)| m.iter().filter(|&&b| b).count())
        .sum()
}

const NX: usize = 5;
const NY: usize = 5;
const CENTER: usize = NX / 2 + NY / 2 * NX;
const BORDERS: [usize; 4] = [CENTER - 1, CENTER + 1, CENTER - NX, CENTER + NX];

fn update_bugs(maps: &mut HashMap<i16, Vec<bool>>, copy: &mut HashMap<i16, Vec<bool>>) {
    let min_level = *maps.keys().min().unwrap();
    let max_level = *maps.keys().max().unwrap();
    copy.insert(min_level - 1, vec![false; NX * NY]);
    copy.insert(max_level + 1, vec![false; NX * NY]);
    maps.insert(min_level - 1, vec![false; NX * NY]);
    maps.insert(max_level + 1, vec![false; NX * NY]);

    std::mem::swap(copy, maps);
    for level in (min_level - 1)..=(max_level + 1) {
        let map = maps.get_mut(&level).unwrap();
        let hole = if let Some(hole) = copy.get(&(level + 1)) {
            [
                hole.iter().step_by(NX).filter(|&&b| b).count(),
                hole.iter().skip(NX - 1).step_by(NX).filter(|&&b| b).count(),
                hole.iter().take(NX).filter(|&&b| b).count(),
                hole.iter().skip(NX * (NY - 1)).filter(|&&b| b).count(),
            ]
        } else {
            [0, 0, 0, 0]
        };

        let borders = if let Some(sublevel) = copy.get(&(level - 1)) {
            BORDERS.map(|i| usize::from(sublevel[i]))
        } else {
            [0, 0, 0, 0]
        };
        let current_copy = copy.get(&level).unwrap();
        for (i, b) in map.iter_mut().enumerate().filter(|&(i, _)| i != CENTER) {
            let mut n = get_neighbours(i).filter(|&p| current_copy[p]).count();
            if i % NX == 0 {
                n += borders[0]
            }
            if i % NX == NX - 1 {
                n += borders[1]
            }
            if i / NX == 0 {
                n += borders[2]
            }
            if i / NX == NY - 1 {
                n += borders[3]
            }

            if let Some(index) = BORDERS.iter().position(|&b| b == i) {
                n += hole[index]
            }

            if n == 1 || (!current_copy[i] && n == 2) {
                *b = true;
            } else {
                *b = false;
            }
        }
    }
}

const DELTAS: [(i8, i8); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

fn get_neighbours(pos: usize) -> impl Iterator<Item = usize> {
    DELTAS
        .iter()
        .filter(move |(dx, dy)| {
            let (x, y) = ((pos % NX) as i8 + dx, (pos / NX) as i8 + dy);
            x >= 0 && x < NX as i8 && y >= 0 && y < NY as i8
        })
        .map(move |(dx, dy)| (pos as i8 + dx + dy * NX as i8) as usize)
}

fn parse(input: &str) -> Vec<bool> {
    input
        .lines()
        .flat_map(|l| l.bytes().map(|b| b == b'#'))
        .collect()
}
