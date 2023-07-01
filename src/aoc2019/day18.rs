use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, VecDeque},
};

pub fn part1(input: &str) -> u32 {
    let nx = input.lines().next().unwrap().len() as i32;
    let map: Vec<_> = input.lines().flat_map(|l| l.chars()).collect();
    let ny = map.len() as i32 / nx;

    let spawn = map
        .iter()
        .position(|&c| c == '@')
        .map(|p| (p as i32 % nx, p as i32 / nx))
        .unwrap();
    let n_keys = map.iter().filter(|&c| c.is_lowercase()).count();
    let mut objects = vec![Object::default(); n_keys * 2];
    let mut obstructed = 0;
    let keys = 2u32.pow(n_keys as u32) - 1;
    (b'a'..b'a' + n_keys as u8)
        .chain(b'A'..b'A' + n_keys as u8)
        .enumerate()
        .for_each(|(i, id)| objects[i].id = id);
    let first = get_starter_robots(
        spawn,
        map.clone(),
        nx,
        ny,
        n_keys,
        &mut objects,
        &mut obstructed,
    );

    for &(obj_index, _) in first.paths.iter() {
        get_distances(obj_index, map.clone(), nx, ny, n_keys, &mut objects);
    }

    objects.push(first);
    let state = State {
        indices: [objects.len() - 1],
        distance: 0,
        obstructed,
        keys,
    };

    find_minimum(state, objects, n_keys)
}

pub fn part2(input: &str) -> u32 {
    let nx = input.lines().next().unwrap().len() as i32;
    let mut map: Vec<_> = input.lines().flat_map(|l| l.chars()).collect();
    let ny = map.len() as i32 / nx;

    let spawn = map
        .iter()
        .position(|&c| c == '@')
        .map(|p| (p as i32 % nx, p as i32 / nx))
        .unwrap();
    let n_keys = map.iter().filter(|&c| c.is_lowercase()).count();
    let mut objects = vec![Object::default(); n_keys * 2];
    let mut obstructed = 0;
    let keys = 2u32.pow(n_keys as u32) - 1;
    (b'a'..b'a' + n_keys as u8)
        .chain(b'A'..b'A' + n_keys as u8)
        .enumerate()
        .for_each(|(i, id)| objects[i].id = id);
    let center = spawn.0 + spawn.1 * nx;
    [center - 1, center, center + 1, center - nx, center + nx]
        .into_iter()
        .for_each(|c| map[c as usize] = '#');
    let starters = [
        (spawn.0 - 1, spawn.1 - 1),
        (spawn.0 - 1, spawn.1 + 1),
        (spawn.0 + 1, spawn.1 - 1),
        (spawn.0 + 1, spawn.1 + 1),
    ]
    .map(|spawn| {
        get_starter_robots(
            spawn,
            map.clone(),
            nx,
            ny,
            n_keys,
            &mut objects,
            &mut obstructed,
        )
    });

    for (obj_index, _) in starters.iter().flat_map(|s| s.paths.clone()) {
        get_distances(obj_index, map.clone(), nx, ny, n_keys, &mut objects)
    }
    objects.extend_from_slice(&starters);
    let len = objects.len();
    let state = State {
        indices: [len - 4, len - 3, len - 2, len - 1],
        distance: 0,
        obstructed,
        keys,
    };

    find_minimum(state, objects, n_keys)
}

fn get_starter_robots(
    spawn: (i32, i32),
    mut map: Vec<char>,
    nx: i32,
    ny: i32,
    n_keys: usize,
    objects: &mut Vec<Object>,
    obstructed_list: &mut u64,
) -> Object {
    let mut starter = Object::default();
    let mut q: VecDeque<((i32, i32), u32, Option<usize>)> = VecDeque::from([(spawn, 0, None)]);
    while let Some((pos, d, obstructing)) = q.pop_front() {
        for m in MOVES {
            let (new_x, new_y) = (pos.0 + m.0, pos.1 + m.1);
            let c = map.get_mut((new_x + new_y * nx) as usize).unwrap();
            if new_x >= 0 && new_x < nx && new_y >= 0 && new_y < ny && *c != '#' {
                let mut new_obstructing = obstructing;
                if c.is_alphabetic() {
                    let index = if c.is_lowercase() {
                        (*c as u8 - b'a') as usize
                    } else {
                        n_keys + (*c as u8 - b'A') as usize
                    };
                    if let Some(obstructing_index) = obstructing {
                        *obstructed_list |= 1u64 << index;
                        objects[obstructing_index].obstructing.push(index);
                    }
                    new_obstructing = Some(index);
                    starter.paths.push((index, d + 1));
                }
                *c = '#';
                q.push_back(((new_x, new_y), d + 1, new_obstructing));
            }
        }
    }
    starter
}

fn get_distances(
    obj_index: usize,
    mut map: Vec<char>,
    nx: i32,
    ny: i32,
    n_keys: usize,
    objects: &mut Vec<Object>,
) {
    let obj_pos = map
        .iter()
        .position(|&c| c as u8 == objects[obj_index].id)
        .map(|p| (p as i32 % nx, p as i32 / nx))
        .unwrap();
    let mut q = VecDeque::from([(obj_pos, 0)]);
    map[(obj_pos.0 + obj_pos.1 * nx) as usize] = '#';
    while let Some((pos, d)) = q.pop_front() {
        for m in MOVES {
            let (new_x, new_y) = (pos.0 + m.0, pos.1 + m.1);
            let c = map.get_mut((new_x + new_y * nx) as usize).unwrap();
            if new_x >= 0 && new_x < nx && new_y >= 0 && new_y < ny && *c != '#' {
                if c.is_alphabetic() {
                    let index = if c.is_lowercase() {
                        (*c as u8 - b'a') as usize
                    } else {
                        n_keys + (*c as u8 - b'A') as usize
                    };
                    objects[obj_index].paths.push((index, d + 1));
                }
                *c = '#';
                q.push_back(((new_x, new_y), d + 1));
            }
        }
    }
}

fn find_minimum<const R: usize>(state: State<R>, objects: Vec<Object>, n_keys: usize) -> u32 {
    let mut q = BinaryHeap::from([state]);
    let mut cache = HashMap::new();
    while let Some(state) = q.pop() {
        if state.keys == 0 {
            return state.distance;
        }
        for i in 0..state.indices.len() {
            for (n_index, d) in objects[state.indices[i]].paths.iter() {
                if state.obstructed & (1u64 << n_index) == 0
                    && (*n_index < n_keys || state.keys & 1u32 << (n_index - n_keys) == 0)
                {
                    let mut new_state = state.clone();
                    if n_index < &n_keys {
                        new_state.keys ^= 1u32 << *n_index;
                    }
                    new_state.indices[i] = *n_index;
                    let c = *cache.get(&new_state.get_key()).unwrap_or(&u32::MAX);
                    if c > state.distance + d {
                        for o in objects[*n_index].obstructing.iter() {
                            new_state.obstructed ^= 1u64 << o;
                        }
                        new_state.obstructed |= 1u64 << n_index;
                        new_state.distance = state.distance + d;
                        cache.insert(new_state.get_key(), new_state.distance);
                        q.push(new_state);
                    }
                }
            }
        }
    }
    panic!("not found")
}

#[derive(Eq, PartialEq, Clone)]
struct State<const R: usize> {
    indices: [usize; R],
    distance: u32,
    obstructed: u64,
    keys: u32,
}

impl<const R: usize> State<R> {
    fn get_key(&self) -> u64 {
        self.indices
            .iter()
            .fold(0, |acc, &i| (acc << 8) | (i as u64))
            | (self.keys as u64) << 32
    }
}

impl<const R: usize> Ord for State<R> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}
impl<const R: usize> PartialOrd for State<R> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const MOVES: [(i32, i32); 4] = [(0, 1), (0, -1), (-1, 0), (1, 0)];

#[derive(Debug, Clone, Default)]
struct Object {
    id: u8,
    paths: Vec<(usize, u32)>,
    obstructing: Vec<usize>,
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
