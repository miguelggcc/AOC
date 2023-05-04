use std::{
    collections::{HashSet, VecDeque},
    fmt::Debug,
    time::Instant,
};

use nom::{
    bytes::complete::tag,
    character::complete,
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    Finish, IResult,
};

pub fn day19(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Can't read input file");
    let time = Instant::now();
    //Part 1
    println!("Sum of quality levels: {}", do_19_part1(&input));

    //Part 2
    println!("Product of first 3 bp: {}", do_19_part2(&input));

    println!("{:?}", time.elapsed());
}

fn do_19_part1(input: &str) -> u32 {
    let bps = parse(input);
    bps.iter()
        .map(|bp| dbg!(find_max_geode(bp, 24)) * bp.id as u32)
        .sum()
}

fn do_19_part2(input: &str) -> u32 {
    let bps = parse(input);
    bps.iter()
        .take(3)
        .map(|bp| dbg!(find_max_geode(bp, 32)))
        .product()
}

fn find_max_geode(bp: &Blueprint, max_time: u8) -> u32 {
    let state_root = State {
        time: max_time,
        ore: 0,
        clay: 0,
        obsidian: 0,
        geode: 0,
        ore_robots: 1,
        clay_robots: 0,
        obsidian_robots: 0,
        geode_robots: 0,
    };
    let mut max_geode = 0;
    let times = (1..max_time + 1)
        .map(|t| t as u32 * (t as u32 - 1) / 2)
        .collect::<Vec<_>>();
    let mut visited = HashSet::new();
    let mut states = VecDeque::from(vec![state_root]);

    while let Some(mut parent_state) = states.pop_front() {
        for state in parent_state.next_states(bp) {
            if !visited.contains(&state.to_bytes()) {
                visited.insert(state.to_bytes());
                max_geode = max_geode.max(state.geode + state.time * state.geode_robots);

                if state.time > 0
                    && bp.geode_robot.1 as u32
                        <= state.obsidian as u32
                            + state.time as u32 * state.obsidian_robots as u32
                            + times[state.time as usize]
                    && max_geode
                        < state.geode
                            + state.time * state.geode_robots
                            + times[state.time as usize] as u8
                {
                    states.push_back(state);
                }
            }
        }
    }
    dbg!(visited.len());
    max_geode as u32
}

fn parse(input: &str) -> Vec<Blueprint> {
    all_consuming(preceded(
        tag("Blueprint "),
        separated_list1(
            pair(complete::multispace1, tag("Blueprint ")),
            parse_blueprint,
        ),
    ))(input)
    .finish()
    .unwrap()
    .1
}

#[derive(Debug)]
struct Blueprint {
    id: u8,
    ore_robot: u8,
    clay_robot: u8,
    obsidian_robot: (u8, u8),
    geode_robot: (u8, u8),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State {
    time: u8,
    ore: u8,
    clay: u8,
    obsidian: u8,
    geode: u8,
    ore_robots: u8,
    clay_robots: u8,
    obsidian_robots: u8,
    geode_robots: u8,
    //path: String
}

impl State {
    fn step(&mut self) {
        self.time -= 1;
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geode += self.geode_robots;
    }
    fn next_states(&mut self, bp: &Blueprint) -> Vec<Self> {
        if self.time == 1 {
            self.step();
            return vec![self.clone()];
        }
        let mut v = vec![];
        if self.ore >= bp.geode_robot.0 && self.obsidian >= bp.geode_robot.1 {
            let mut other_state = self.clone();
            other_state.step();
            other_state.ore -= bp.geode_robot.0;
            other_state.obsidian -= bp.geode_robot.1;
            other_state.geode_robots += 1;
            v.push(other_state);
        } else {
            if self.ore >= bp.obsidian_robot.0 && self.clay >= bp.obsidian_robot.1 {
                let mut other_state = self.clone();
                other_state.step();
                other_state.ore -= bp.obsidian_robot.0;
                other_state.clay -= bp.obsidian_robot.1;
                other_state.obsidian_robots += 1;
                v.push(other_state);
            }
            if self.ore >= bp.clay_robot && self.clay < bp.obsidian_robot.1 {
                let mut other_state = self.clone();
                other_state.step();
                other_state.ore -= bp.clay_robot;
                other_state.clay_robots += 1;
                v.push(other_state);
            }
            if self.ore >= bp.ore_robot && self.clay < bp.geode_robot.0 {
                let mut other_state = self.clone();
                other_state.step();
                other_state.ore -= bp.ore_robot;
                other_state.ore_robots += 1;
                v.push(other_state);
            }
            self.step();
            v.push(self.clone());
        }
        v
    }
    fn to_bytes(&self) -> u64 {
        u64::from_be_bytes([
            self.time,
            self.ore,
            self.clay,
            self.obsidian,
            self.geode,
            self.ore_robots,
            self.clay_robots,
            self.obsidian_robots + (self.geode_robots << 4),
        ])
    }
}

fn parse_blueprint(input: &str) -> IResult<&str, Blueprint> {
    let (input, id) = terminated(complete::u8, tag(":"))(input)?;
    let (input, ore_robot) = delimited(
        pair(complete::multispace1, tag("Each ore robot costs ")),
        complete::u8,
        tag(" ore."),
    )(input)?;
    let (input, clay_robot) = delimited(
        pair(complete::multispace1, tag("Each clay robot costs ")),
        complete::u8,
        tag(" ore."),
    )(input)?;
    let (input, obsidian_robot) = delimited(
        pair(complete::multispace1, tag("Each obsidian robot costs ")),
        separated_pair(complete::u8, tag(" ore and "), complete::u8),
        tag(" clay."),
    )(input)?;
    let (input, geode_robot) = delimited(
        pair(complete::multispace1, tag("Each geode robot costs ")),
        separated_pair(complete::u8, tag(" ore and "), complete::u8),
        tag(" obsidian."),
    )(input)?;

    Ok((
        input,
        Blueprint {
            id,
            ore_robot,
            clay_robot,
            obsidian_robot,
            geode_robot,
        },
    ))
}

#[cfg(test)]
mod tests {

    use super::do_19_part1;
    use super::do_19_part2;

    #[test]
    fn part_1() {
        let input = "Blueprint 1:
  Each ore robot costs 4 ore.
  Each clay robot costs 2 ore.
  Each obsidian robot costs 3 ore and 14 clay.
  Each geode robot costs 2 ore and 7 obsidian.
      
Blueprint 2:
  Each ore robot costs 2 ore.
  Each clay robot costs 3 ore.
  Each obsidian robot costs 3 ore and 8 clay.
  Each geode robot costs 3 ore and 12 obsidian.";

        assert_eq!(do_19_part1(input), 33);
        assert_eq!(do_19_part2(input), 1707);
    }
}
