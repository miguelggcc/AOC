use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Debug,
    time::Instant,
};

use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete,
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{preceded, tuple},
    Finish, IResult,
};

pub fn day16(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Can't read input file");
    let time = Instant::now();
    //Part 1
    println!("Max pressure: {}", do_16_part1(&input));
    println!("Max pressure: {}", do_16_part1(&input));
    println!("Max pressure: {}", do_16_part1(&input));
    println!("Max pressure: {}", do_16_part1(&input));
    println!("Max pressure: {}", do_16_part1(&input));
    println!("Max pressure: {}", do_16_part1(&input));
    println!("Max pressure: {}", do_16_part1(&input));
    println!("Max pressure: {}", do_16_part1(&input));

    //Part 2
    //println!("Part 2, decoder key: {}", do_16_part2(&input, 4000000));

    println!("{:?}", time.elapsed());
}

fn do_16_part1(input: &str) -> u32 {
    let lines = input
        .lines()
        .map(|line| all_consuming(parse_line)(line).finish().unwrap().1);
    let mut relevant_valves = vec![];
    let mut ids = HashMap::new();
    let mut root_index = usize::MAX; //error if "AA" is not found

    let mut valves: Vec<_> = lines
        .enumerate()
        .map(|(i, (id, rate, children))| {
            ids.insert(id.clone(), i);

            if rate > 0 {
                relevant_valves.push(i);
            }
            if id == "AA" {
                root_index = i;
            }

            Valve {
                rate,
                children,
                paths: vec![],
            }
        })
        .collect();

    let mut distances = vec![0; valves.len()];

    for i in 0..valves.len(){
        valves[i].paths = relevant_valves
            .iter()
            .filter(|rv_index| rv_index != &&i)
            .map(|rv_index| {
                distances = vec![0; ids.len()];
                let mut walking_indices = VecDeque::from(vec![i]);

                let mut visited = HashSet::new();

                'w: while let Some(walking_index) = walking_indices.pop_front() {
                    visited.insert(walking_index);
                    let dist = distances[walking_index];

                    for c_i in valves[walking_index].children.iter().map(|id| ids.get(id).unwrap()) {
                        if !visited.contains(c_i) {
                            distances[*c_i] = dist + 1;
                            if c_i == rv_index {
                                break 'w;
                            }
                            walking_indices.push_back(*c_i);
                        }
                    }
                }
                Path {
                    distance: distances[*rv_index],
                    valve: *rv_index,
                }
            })
            .collect();
    };
    let walker_root = Walker {
        node_index: root_index,
        rate: 0,
        pressure: 0,
        time: 30,
        valves_unopen: HashSet::from_iter(relevant_valves),
        //path: vec![("AA".to_string(), 30, 0)],
    };
    let mut walkers = VecDeque::from(vec![walker_root]);
    let mut max_pressure = 0;
    let mut total = 0u64;
    while let Some(walker) = walkers.pop_front() {
        
            for path in valves[walker.node_index]
                .paths
                .iter()
                .filter(|path| {
                    path.distance +1  < walker.time && walker.valves_unopen.contains(&path.valve)
                })
            {
                let mut child_walker = walker.clone();

                child_walker.time -= path.distance;
                child_walker.pressure += walker.rate * path.distance;
                child_walker.node_index = path.valve;

                    child_walker.valves_unopen.remove(&path.valve);
                    child_walker.time -= 1;
                    child_walker.pressure += walker.rate;
                    child_walker.rate += valves[path.valve].rate;
                    /*child_walker.path.push((
                        valves[valves_i[path.valve]].id.clone(),
                        child_walker.time,
                        child_walker.pressure,
                    ));*/
                
                max_pressure = max_pressure.max(child_walker.pressure + child_walker.rate * child_walker.time);
                if child_walker.time > 0 && !child_walker.valves_unopen.is_empty() {
                walkers.push_back(child_walker);
                total += 1;
            }
        }
    }
    dbg!(total);
    max_pressure
}

fn do_16_part2(input: &str, max_coord: usize) -> u64 {
    panic!("couldn't find beacon")
}

#[derive(Debug)]
struct Valve {
    rate: u32,
    children: Vec<String>,
    paths: Vec<Path>,
}

#[derive(Debug)]
struct Path {
    valve: usize,
    distance: u32,
}

#[derive(Debug, Clone)]
struct Walker {
    node_index: usize,
    rate: u32,
    pressure: u32,
    time: u32,
    valves_unopen: HashSet<usize>,
    //path: Vec<(String, u32, u32)>,
}

fn parse_line(input: &str) -> IResult<&str, (String, u32, Vec<String>)> {
    let (input, (id, _, rate, _, valves)) = preceded(
        tag("Valve "),
        tuple((
            is_a("ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
            tag(" has flow rate="),
            complete::u32,
            alt((
                tag("; tunnels lead to valves "),
                tag("; tunnel leads to valve "),
            )),
            separated_list1(tag(", "), is_a("ABCDEFGHIJKLMNOPQRSTUVWXYZ")),
        )),
    )(input)?;
    Ok((
        input,
        (
            id.to_string(),
            rate,
            valves.iter().map(|v| v.to_string()).collect(),
        ),
    ))
}

#[cfg(test)]
mod tests {

    use super::do_16_part1;
    //use super::do_16_part2;

    #[test]
    fn part_1() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

        assert_eq!(do_16_part1(input), 1651);
        //assert_eq!(do_16_part2(input, 20), 56000011)
    }
}
