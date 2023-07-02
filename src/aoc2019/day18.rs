use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, VecDeque},
};

pub fn part1(input: &str) -> u32 {
    let (map, spawn) = Map::parse(input);

    let n_keys = map.v.iter().filter(|&c| c.is_lowercase()).count();
    let mut objects: Vec<_> = (0..n_keys as u8)
        .flat_map(|i| [Object::new(i + b'a'), Object::new(i + b'A')])
        .collect();
    let mut obstructed = 0;
    let first = get_robots(spawn, map.clone(), &mut objects, &mut obstructed);
    for &(obj_index, _) in first.paths.iter() {
        get_distances(obj_index, map.clone(), &mut objects);
    }
    objects.push(first);

    let state = State {
        indices: [objects.len() - 1],
        distance: 0,
        obstructed,
        keys: 2u32.pow(n_keys as u32) - 1,
    };
    find_minimum(state, objects)
}

pub fn part2(input: &str) -> u32 {
    let (mut map, spawn) = Map::parse(input);

    let n_keys = map.v.iter().filter(|&c| c.is_lowercase()).count();
    let mut objects: Vec<_> = (0..n_keys as u8)
        .flat_map(|i| [Object::new(i + b'a'), Object::new(i + b'A')])
        .collect();
    let mut obstructed = 0;
    MOVES
        .iter()
        .for_each(|(dx, dy)| map.v[(spawn.0 + dx + (spawn.1 + dy) * map.nx) as usize] = '#');
    let starters = [
        (spawn.0 - 1, spawn.1 - 1),
        (spawn.0 - 1, spawn.1 + 1),
        (spawn.0 + 1, spawn.1 - 1),
        (spawn.0 + 1, spawn.1 + 1),
    ]
    .map(|spawn| get_robots(spawn, map.clone(), &mut objects, &mut obstructed));

    for (obj_index, _) in starters.iter().flat_map(|s| s.paths.clone()) {
        get_distances(obj_index, map.clone(), &mut objects)
    }
    objects.extend_from_slice(&starters);

    let len = objects.len();
    let state = State {
        indices: [len - 4, len - 3, len - 2, len - 1],
        distance: 0,
        obstructed,
        keys: 2u32.pow(n_keys as u32) - 1,
    };
    find_minimum(state, objects)
}

const MOVES: [(i32, i32); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];

fn get_robots(
    spawn: (i32, i32),
    mut map: Map,
    objects: &mut [Object],
    obstructed: &mut u64,
) -> Object {
    let mut starter = Object::default();
    let mut q: VecDeque<((i32, i32), u32, Option<usize>)> = VecDeque::from([(spawn, 0, None)]);
    while let Some((pos, d, obstructing)) = q.pop_front() {
        for m in MOVES {
            let new_pos = (pos.0 + m.0, pos.1 + m.1);
            let c = map.get_mut(new_pos);
            if *c != '#' {
                let mut new_obstructing = obstructing;
                if c.is_alphabetic() {
                    let index = get_index(*c);
                    if let Some(obstructing_index) = obstructing {
                        *obstructed |= 1u64 << index;
                        objects[obstructing_index].obstructing |= 1u64 << index;
                    }
                    new_obstructing = Some(index);
                    starter.paths.push((index, d + 1));
                }
                *c = '#';
                q.push_back((new_pos, d + 1, new_obstructing));
            }
        }
    }
    starter
}

fn get_distances(obj_index: usize, mut map: Map, objects: &mut [Object]) {
    let obj_pos = map
        .v
        .iter()
        .position(|&c| c as u8 == objects[obj_index].id)
        .unwrap();
    let mut q = VecDeque::from([(obj_pos as i32, 0)]);
    *map.v.get_mut(obj_pos).unwrap() = '#';
    while let Some((pos, d)) = q.pop_front() {
        for m in MOVES {
            let new_pos = pos + m.0 + m.1 * map.nx;
            let c = map.v.get_mut(new_pos as usize).unwrap();
            if *c != '#' {
                if c.is_alphabetic() {
                    objects[obj_index].paths.push((get_index(*c), d + 1));
                }
                *c = '#';
                q.push_back((new_pos, d + 1));
            }
        }
    }
}

fn find_minimum<const I: usize>(state: State<I>, objects: Vec<Object>) -> u32 {
    let mut heap = BinaryHeap::from([state]);
    let mut cache = HashMap::new();
    while let Some(state) = heap.pop() {
        if state.keys == 0 {
            return state.distance;
        }
        for i in 0..I {
            for (n_index, d) in objects[state.indices[i]].paths.iter() {
                if state.obstructed & (1u64 << n_index) == 0
                    && (n_index % 2 == 0 || state.keys & 1u32 << (n_index / 2) == 0)
                {
                    let mut new_state = state.clone();
                    if n_index % 2 == 0 {
                        new_state.keys ^= 1u32 << (*n_index / 2);
                    }
                    new_state.indices[i] = *n_index;
                    let c = *cache.get(&new_state.get_key()).unwrap_or(&u32::MAX);
                    if c > state.distance + d {
                        new_state.obstructed ^= objects[*n_index].obstructing;

                        new_state.obstructed |= 1u64 << n_index;
                        new_state.distance += d;
                        cache.insert(new_state.get_key(), new_state.distance);
                        heap.push(new_state);
                    }
                }
            }
        }
    }
    panic!("not found")
}

#[derive(Eq, PartialEq, Clone)]
struct State<const I: usize> {
    indices: [usize; I],
    distance: u32,
    obstructed: u64,
    keys: u32,
}

impl<const I: usize> State<I> {
    fn get_key(&self) -> u64 {
        self.indices
            .iter()
            .fold(0, |acc, &i| (acc << 8) | (i as u64))
            | (self.keys as u64) << 32
    }
}

impl<const I: usize> Ord for State<I> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}
impl<const I: usize> PartialOrd for State<I> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Default)]
struct Object {
    id: u8,
    paths: Vec<(usize, u32)>,
    obstructing: u64,
}

impl Object {
    fn new(id: u8) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

fn get_index(c: char) -> usize {
    if c.is_lowercase() {
        2 * (c as u8 - b'a') as usize
    } else {
        1 + 2 * (c as u8 - b'A') as usize
    }
}

#[derive(Clone)]
struct Map {
    v: Vec<char>,
    nx: i32,
}

impl Map {
    fn parse(input: &str) -> (Self, (i32, i32)) {
        let nx = input.lines().next().unwrap().len() as i32;
        let v: Vec<_> = input.lines().flat_map(|l| l.chars()).collect();

        let spawn = v
            .iter()
            .position(|&c| c == '@')
            .map(|p| (p as i32 % nx, p as i32 / nx))
            .unwrap();
        (Self { v, nx }, spawn)
    }
    fn get_mut(&mut self, pos: (i32, i32)) -> &mut char {
        self.v.get_mut((pos.0 + pos.1 * self.nx) as usize).unwrap()
    }
}

#[cfg(test)]
mod day18 {

    use super::*;

    #[test]
    fn part_1() {
        let input = "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################";
        assert_eq!(part1(input), 86);
    }
    #[test]
    fn part_2() {
        let input = "###############
#d.ABC.#.....a#
######...######
######.@.######
######...######
#b.....#.....c#
###############";

        assert_eq!(part2(input), 24);
    }
}
