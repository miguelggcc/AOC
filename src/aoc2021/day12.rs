use std::collections::{HashMap, VecDeque};

pub fn part1(input: &str) -> u32 {
    let (caves, start, end) = parse(input);
    get_paths(caves, start, end, true)
}

pub fn part2(input: &str) -> u32 {
    let (caves, start, end) = parse(input);
    get_paths(caves, start, end, false)
}

fn get_paths(caves: Vec<Cave>, start: usize, end: usize, once: bool) -> u32 {
    let mut q = VecDeque::from([(start, 1 << start, once)]);
    let mut paths = 0;

    while let Some((index, visited, has_repeated)) = q.pop_front() {
        if index == end {
            paths += 1;
            continue;
        }
        for nindex in caves[index]
            .children
            .iter()
            .filter(|nindex| nindex != &&start)
        {
            if caves[*nindex].small {
                if visited & (1u64 << *nindex) == 0 {
                    q.push_back((*nindex, visited ^ 1u64 << nindex, has_repeated));
                } else if !has_repeated && nindex != &end {
                    q.push_back((*nindex, visited, true));
                }
                continue;
            }
            q.push_back((*nindex, visited, has_repeated));
        }
    }
    paths
}

fn parse(input: &str) -> (Vec<Cave>, usize, usize) {
    let mut ids = HashMap::new();
    let mut children = HashMap::new();
    let mut index = 0;
    input
        .lines()
        .map(|l| l.split('-').collect::<Vec<_>>())
        .for_each(|pair| {
            pair.iter().for_each(|&c| {
                if !ids.contains_key(c) {
                    ids.insert(c.to_string(), index);
                    index += 1;
                }
            });
            let ids0 = *ids.get(pair[0]).unwrap();
            let ids1 = *ids.get(pair[1]).unwrap();
            children
                .entry(pair[0].to_string())
                .or_insert(vec![])
                .push(ids1);
            children
                .entry(pair[1].to_string())
                .or_insert(vec![])
                .push(ids0);
        });
    let mut caves: Vec<_> = children
        .drain()
        .map(|(id, children)| {
            let small = id.chars().next().unwrap().is_lowercase();
            Cave {
                id,
                small,
                children,
            }
        })
        .collect();
    caves.sort_by_key(|c| ids.get(&c.id).unwrap());
    let start = caves
        .iter()
        .position(|c| c.id == "start")
        .expect("no 'start'");
    let end = caves.iter().position(|c| c.id == "end").expect("no ' end'");
    (caves, start, end)
}

#[derive(Debug)]
struct Cave {
    id: String,
    small: bool,
    children: Vec<usize>,
}

#[cfg(test)]
mod day12 {

    use super::*;

    const INPUT: &'static str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), 10);
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT), 36);
    }
}
