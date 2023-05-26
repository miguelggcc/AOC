use std::collections::{hash_map::Entry, HashMap, VecDeque};

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
        for nindex in caves[index].children.iter() {
            if caves[*nindex].small {
                if visited & (1u32 << *nindex) == 0 {
                    q.push_back((*nindex, visited ^ 1u32 << nindex, has_repeated));
                } else if !has_repeated && nindex != &start {
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
    let mut cave_map: HashMap<String, (usize, Vec<_>)> = HashMap::new();
    let mut index = 0;
    input
        .lines()
        .map(|l| l.split('-').collect::<Vec<_>>())
        .for_each(|pair| {
            let index_left = match cave_map.entry(pair[0].to_string()) {
                Entry::Occupied(o) => (*o.get()).0,
                Entry::Vacant(v) => {
                    v.insert((index, vec![]));
                    index += 1;
                    index - 1
                }
            };
            let index_right = match cave_map.entry(pair[1].to_string()) {
                Entry::Occupied(mut o) => {
                    (*o.get_mut()).1.push(index_left);
                    (*o.get()).0
                }
                Entry::Vacant(v) => {
                    v.insert((index, vec![index_left]));
                    index += 1;
                    index - 1
                }
            };
            (*cave_map.get_mut(pair[0]).unwrap()).1.push(index_right);
        });
    let mut caves = vec![Cave::default(); cave_map.len()];
    cave_map.drain().for_each(|(id, (index, children))| {
        let small = id.chars().next().unwrap().is_lowercase();
        caves[index] = Cave {
            id,
            small,
            children,
        }
    });
    let start = caves
        .iter()
        .position(|c| c.id == "start")
        .expect("no 'start'");
    let end = caves.iter().position(|c| c.id == "end").expect("no 'end'");
    (caves, start, end)
}

#[derive(Default, Clone)]
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
