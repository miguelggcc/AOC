use std::collections::{ HashMap, HashSet};

pub fn part1(input: &str) -> u32 {
    let mut map: Vec<_> = input
        .lines()
        .flat_map(|l| l.bytes().map(|b| b == b'#'))
        .collect();
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
    let map: Vec<_> = input
        .lines()
        .flat_map(|l| l.bytes().map(|b| b == b'#'))
        .collect();
    let mut maps = HashMap::new();
    maps.insert(0i16, map);
    maps.insert(-1i16, vec![false; NX * NY]);
    maps.insert(1i16, vec![false; NX * NY]);

    for _ in 0..200 {
        let copy = maps.clone();
        update_bugs(0, &mut maps, &copy);
    }
    maps.into_iter()
        .map(|(_, m)| m.iter().filter(|&&b| b).count())
        .sum()

        /*let mut maps: Vec<_> = maps.into_iter().collect();
        maps.sort_by_key(|(l,_)|*l);
        maps.into_iter().map(|(l,m)|{ println!("\nlevel : {l}"); m.chunks(NX).for_each(|c|println!("{:?}",c.iter().map(|&b|if b{'#'}else{'.'}).collect::<String>()));
           m.iter().filter(|&&b|b).count()}).sum()*/
}

const NX: usize = 5;
const NY: usize = 5;

fn update_bugs(level: i16, maps: &mut HashMap<i16, Vec<bool>>, copy: &HashMap<i16, Vec<bool>>) {
    if let Some(map) = maps.get_mut(&level) {
        let current_copy = map.clone();

        let (left, right, up, down) = if let Some(hole) = copy.get(&(level + 1)) {
            (
                hole.iter().step_by(NX).filter(|&&b| b).count(),
                hole.iter().skip(NX - 1).step_by(NX).filter(|&&b| b).count(),
                hole.iter().take(NX).filter(|&&b| b).count(),
                hole.iter().skip(NX * (NY - 1)).filter(|&&b| b).count(),
            )
        } else {
            (0, 0, 0, 0)
        };

        let (west, east, north, south) = if let Some(borders) = copy.get(&(level - 1)) {
            (
                usize::from(borders[NX / 2 - 1 + NY / 2 * NX]),
                usize::from(borders[NX / 2 + 1 + NY / 2 * NX]),
                usize::from(borders[NX / 2 + (NY / 2 - 1) * NX]),
                usize::from(borders[NX / 2 + (NY / 2 + 1) * NX]),
            )
        } else {
            (0, 0, 0, 0)
        };

        for (i, b) in map
            .iter_mut()
            .enumerate()
            .filter(|&(i, _)| i != NX / 2 + NY / 2 * NX)
        {
            let mut n = get_neighbours(i).filter(|&p| current_copy[p]).count();
            if i % NX == 0 {
                n += west
            }
            if i % NX == NX - 1 {
                n += east
            }
            if i / NX == 0 {
                n += north
            }
            if i / NX == NY - 1 {
                n += south
            }

            if i == NX / 2 - 1 + NY / 2 * NX {
                n += left
            }
            if i == NX / 2 + 1 + NY / 2 * NX {
                n += right
            }
            if i == NX / 2 + (NY / 2 - 1) * NX {
                n += up
            }
            if i == NX / 2 + (NY / 2 + 1) * NX {
                n += down
            }

            if n == 1 || (!*b && n == 2) {
                *b = true;
            } else {
                *b = false;
            }
        }
        if level == 0 {
            update_bugs(level - 1, maps, copy);
            update_bugs(level + 1, maps, copy);
        } else {
            update_bugs(level + level.signum(), maps, copy);
        }
    } else {
        maps.insert(level, vec![false; NX * NY]);
    }
}

type Point = (i8, i8);
const DELTAS: [Point; 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

fn get_neighbours(pos: usize) -> impl Iterator<Item = usize> {
    DELTAS
        .iter()
        .filter(move |(dx, dy)| {
            let (x, y) = ((pos % NX) as i8 + dx, (pos / NX) as i8 + dy);
            x >= 0 && x < NX as i8 && y >= 0 && y < NY as i8
        })
        .map(move |(dx, dy)| (pos as i8 + dx + dy * NX as i8) as usize)
}

#[cfg(test)]
mod day24 {

    use super::*;

    const INPUT: &'static str = "....#
#..#.
#.?##
..#..
#....";

    #[test]
    fn part_1() {
        assert_eq!(part2(INPUT), 99);
    }
}
