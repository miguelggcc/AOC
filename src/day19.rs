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
        .map(|bp| find_max_geode(bp, 24) * bp.id as u32)
        .sum()
}

fn do_19_part2(input: &str) -> u32 {
    let bps = parse(input);
    bps.iter()
        .take(3)
        .map(|bp| find_max_geode(bp, 32))
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
    };
    let mut max_geode = 0;
    let times = (1..max_time + 2)
        .map(|t| t as u16 * (t.saturating_sub(1) as u16) / 2)
        .collect::<Vec<_>>();
    let mut visited = HashSet::new();
    let mut states = VecDeque::from(vec![(state_root, 0)]);

    while let Some((mut parent_state, previous_missed_robots)) = states.pop_front() {
        for (state, missed_robots) in parent_state
            .next_states(bp, previous_missed_robots)
            .into_iter()
            .flatten()
        {
            if !visited.contains(&state.to_bytes()) {
                let possible_best = state.geode as u16 + times[state.time as usize] as u16;

                visited.insert(state.to_bytes());
                max_geode = max_geode.max(state.geode);

                if state.time > 0
                    && bp.geode_robot.1 as u16
                        <= state.obsidian as u16
                            + state.time as u16 * state.obsidian_robots as u16
                            + times[state.time as usize]
                    && (max_geode as u16) < possible_best
                {
                    states.push_back((state, missed_robots));
                }
            }
        }
    }
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
    max_ore: u8,
}

#[derive(Debug, Clone)]
struct State {
    time: u8,
    ore: u8,
    clay: u8,
    obsidian: u8,
    geode: u8,
    ore_robots: u8,
    clay_robots: u8,
    obsidian_robots: u8,
}

impl State {
    fn step(&mut self) {
        self.time -= 1;
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
    }
    fn next_states(
        &mut self,
        bp: &Blueprint,
        previous_missed_robots: u8,
    ) -> Vec<Option<(Self, u8)>> {
        if self.time == 1 {
            return vec![];
        }
        let mut v = vec![];
        let can_build_obsidian_robot =
            self.ore >= bp.obsidian_robot.0 && self.clay >= bp.obsidian_robot.1;
        let can_build_clay_robot = self.ore >= bp.clay_robot;
        let can_build_ore_robot = self.ore >= bp.ore_robot;

        if self.ore >= bp.geode_robot.0 && self.obsidian >= bp.geode_robot.1 {
            let mut other_state = self.clone();
            other_state.step();
            other_state.ore -= bp.geode_robot.0;
            other_state.obsidian -= bp.geode_robot.1;
            other_state.geode += other_state.time;
            v.push(Some((other_state, 0)));
        } else {
            if can_build_obsidian_robot {
                if previous_missed_robots & 1u8 << 2 != 0 {
                    v.push(None)
                } else {
                    let mut other_state = self.clone();
                    other_state.step();
                    other_state.ore -= bp.obsidian_robot.0;
                    other_state.clay -= bp.obsidian_robot.1;
                    other_state.obsidian_robots += 1;
                    v.push(Some((other_state, 0)));
                }
            }
            if can_build_clay_robot && self.clay < bp.obsidian_robot.1 {
                if previous_missed_robots & 1u8 << 1 != 0 {
                    v.push(None)
                } else {
                    let mut other_state = self.clone();
                    other_state.step();
                    other_state.ore -= bp.clay_robot;
                    other_state.clay_robots += 1;
                    v.push(Some((other_state, 0)));
                }
            }
            if can_build_ore_robot && self.ore_robots < bp.max_ore {
                if previous_missed_robots & 1u8 << 0 != 0 {
                    v.push(None)
                } else {
                    let mut other_state = self.clone();
                    other_state.step();
                    other_state.ore -= bp.ore_robot;
                    other_state.ore_robots += 1;
                    v.push(Some((other_state, 0)));
                }
            }

            self.step();
            let missed_robots = (u8::from(can_build_obsidian_robot) << 2)
                + (u8::from(can_build_clay_robot) << 1)
                + u8::from(can_build_ore_robot);
            //If this current branch in which no robots were created could have created a robot and then a
            //robot is created in the next state, then it's inefficient and should be purged

            v.push(Some((self.clone(), missed_robots)));
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
            self.obsidian_robots,
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
            max_ore: ore_robot
                .max(clay_robot)
                .max(obsidian_robot.0)
                .max(geode_robot.0),
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
        assert_eq!(do_19_part2(input), 3348);
    }
}
