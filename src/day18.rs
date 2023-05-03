use std::{
    collections::{HashSet, VecDeque},
    time::Instant,
};

use nom::{
    character::complete,
    combinator::{all_consuming, map},
    multi::separated_list1,
    Finish, IResult,
};

pub fn day18(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Can't find input file");
    //Part 1
    let time = Instant::now();
    println!("Exterior surface {}", do_day18_part1(&input));
    //Part 2
    println!("Part2: Exterior surface {}", do_day18_part2(&input));
    println!("{:?}", time.elapsed());
}

fn do_day18_part1(input: &str) -> usize {
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

fn do_day18_part2(input: &str) -> usize {
    let cubes: HashSet<_> = input
        .lines()
        .map(|line| all_consuming(parse_line)(line).finish().unwrap().1)
        .collect();

    let bounds = cubes.iter().fold(
        Bounds {
            min_x: i32::MAX,
            min_y: i32::MAX,
            min_z: i32::MAX,
            max_x: 0,
            max_y: 0,
            max_z: 0,
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

    let mut steam = HashSet::new();
    while let Some(cube) = q.pop_front() {
        for nc in cube
            .get_neighbours_bounded(&bounds)
            .into_iter()
            .filter(|nb| !cubes.contains(nb))
        {
            if !steam.contains(&nc) {
                steam.insert(nc.clone());
                q.push_back(nc);
            }
        }
    }

    cubes
        .iter()
        .map(|c| {
            c.get_neighbours()
                .iter()
                .filter(|nc| steam.contains(nc))
                .count()
        })
        .sum()
}

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
    fn get_neighbours_bounded(&self, b: &Bounds) -> impl Iterator<Item=Cube> {
        let mut v = Vec::with_capacity(6);
        if self.x > b.min_x - 1 {
            v.push(Self::new(self.x - 1, self.y, self.z));
        }
        if self.y > b.min_y - 1 {
            v.push(Self::new(self.x, self.y - 1, self.z));
        }
        if self.z > b.min_z - 1 {
            v.push(Self::new(self.x, self.y, self.z - 1));
        }
        if self.x < b.max_x + 1 {
            v.push(Self::new(self.x + 1, self.y, self.z));
        }
        if self.y < b.max_y + 1 {
            v.push(Self::new(self.x, self.y + 1, self.z));
        }
        if self.z < b.max_z + 1 {
            v.push(Self::new(self.x, self.y, self.z + 1));
        }
        v.into_iter()
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
    use super::do_day18_part1;
    use super::do_day18_part2;

    #[test]
    fn part_1() {
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

        assert_eq!(do_day18_part1(input), 64);
        assert_eq!(do_day18_part2(input), 58);
    }
}
