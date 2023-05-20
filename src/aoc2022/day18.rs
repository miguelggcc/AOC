use std::collections::{HashSet, VecDeque};

use nom::{
    character::complete,
    combinator::{all_consuming, map},
    multi::separated_list1,
    Finish, IResult,
};

pub fn part1(input: &str) -> usize {
    let cubes: HashSet<_> = input
        .lines()
        .map(|line| all_consuming(parse_line)(line).finish().unwrap().1)
        .collect();

    cubes
        .iter()
        .map(|c| {
            c.get_neighbours()
                .iter()
                .filter(|nc| !cubes.contains(nc))
                .count()
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let cubes: HashSet<_> = input
        .lines()
        .map(|line| all_consuming(parse_line)(line).finish().unwrap().1)
        .collect();

    let bounds = cubes.iter().fold(
        Bounds {
            min_x: i32::MAX,
            min_y: i32::MAX,
            min_z: i32::MAX,
            max_x: i32::MIN,
            max_y: i32::MIN,
            max_z: i32::MIN,
        },
        |b, c| Bounds {
            min_x: b.min_x.min(c.x),
            min_y: b.min_y.min(c.y),
            min_z: b.min_z.min(c.z),
            max_x: b.max_x.max(c.x),
            max_y: b.max_y.max(c.y),
            max_z: b.max_z.max(c.z),
        },
    );

    let mut q = VecDeque::new();
    q.push_back(Cube::new(bounds.min_x, bounds.min_y, bounds.min_z));

    let mut surface = 0;
    let mut steam = HashSet::new();

    while let Some(cube) = q.pop_front() {
        cube.get_neighbours()
            .into_iter()
            .filter(|nc| nc.within_bounds(&bounds))
            .for_each(|nc| {
                if cubes.contains(&nc) {
                    surface += 1;
                } else if !steam.contains(&nc) {
                    steam.insert(nc.clone());
                    q.push_back(nc);
                }
            })
    }
    surface
}

#[inline(always)]
fn parse_line(input: &str) -> IResult<&str, Cube> {
    map(
        separated_list1(complete::char(','), complete::i32),
        |coords| Cube {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        },
    )(input)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
    fn get_neighbours(&self) -> Vec<Cube> {
        vec![
            Self::new(self.x + 1, self.y, self.z),
            Self::new(self.x - 1, self.y, self.z),
            Self::new(self.x, self.y + 1, self.z),
            Self::new(self.x, self.y - 1, self.z),
            Self::new(self.x, self.y, self.z + 1),
            Self::new(self.x, self.y, self.z - 1),
        ]
    }
    fn within_bounds(&self, b: &Bounds) -> bool {
        self.x >= b.min_x - 1
            && self.x <= b.max_x + 1
            && self.y >= b.min_y - 1
            && self.y <= b.max_y + 1
            && self.z >= b.min_z - 1
            && self.z <= b.max_z + 1
    }
}

#[derive(Debug)]
struct Bounds {
    min_x: i32,
    min_y: i32,
    min_z: i32,
    max_x: i32,
    max_y: i32,
    max_z: i32,
}

#[cfg(test)]
mod tests {
    use super::part1;
    use super::part2;

    #[test]
    fn part_1_2() {
        let input = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

        assert_eq!(part1(input), 64);
        assert_eq!(part2(input), 58);
    }
}
