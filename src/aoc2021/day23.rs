use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

pub fn part1(input: &str) -> u32 {
    get_energy::<8>(&input)
}

pub fn part2(input: &str) -> u32 {
    let (i1, i2) = input.split_once("  ").unwrap();
    let insert = "  #D#C#B#A#\n  #D#B#A#C#\n  ";
    get_energy::<16>(&[i1, insert, i2].join(""))
}

fn get_energy<const R: usize>(input: &str) -> u32 {
    let mut rooms = [vec![], vec![], vec![], vec![]];
    let mut cache = HashMap::new();
    input[14 * 2..].lines().rev().skip(1).for_each(|l| {
        l.trim()
            .split('#')
            .filter(|s| !s.is_empty())
            .enumerate()
            .for_each(|(i, a)| rooms[i].push(a.chars().next().unwrap() as u8 - b'A' + 1))
    });
    let l = rooms[0].len();
    let mut heap: BinaryHeap<State<R>> = BinaryHeap::from([State {
        rooms: rooms
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
        hall: [0; 7],
        energy: 0,
    }]);
    while let Some(state) = heap.pop() {
        if state
            .rooms
            .chunks(l)
            .enumerate()
            .all(|(i, room)| room.iter().all(|&ar| ar == i as u8 + 1))
        {
            return state.energy;
        }
        for (i, a) in state.hall.iter().enumerate().filter(|(_, a)| **a != 0) {
            let index = l * (*a as usize - 1);
            if state.rooms[index..index + 1]
                .iter()
                .all(|ar| ar == a || ar == &0)
            {
                if let Some(distance) = distance_room(i, *a, &state.hall) {
                    let mut new_state = state.clone();
                    new_state.hall[i] = 0;
                    let zero = new_state.rooms[index..index + l]
                        .iter()
                        .position(|&ar| ar == 0)
                        .unwrap_or(l - 1);
                    new_state.rooms[index + zero] = *a;
                    new_state.energy +=
                        ((l - zero - 1) as u32 + distance) * 10u32.pow(*a as u32 - 1);
                    let c = cache.get(&new_state.get_key()).unwrap_or(&u32::MAX);
                    if c > &new_state.energy {
                        cache.insert(new_state.get_key(), new_state.energy);
                        heap.push(new_state);
                    }
                }
            }
        }
        for (i, room) in state.rooms.chunks(l).enumerate() {
            if room.iter().any(|&a| a != 0 && a != i as u8 + 1) {
                let possible_pos = distance_row(i, &state.hall);
                for (pos, distance) in possible_pos {
                    let mut new_state = state.clone();
                    let last = new_state.rooms[l * i..l * i + l]
                        .iter()
                        .rposition(|&ar| ar != 0)
                        .unwrap();
                    let a = new_state.rooms[l * i + last];
                    new_state.rooms[l * i + last] = 0;
                    new_state.hall[pos] = a;
                    new_state.energy +=
                        ((l - 1 - last) as u32 + distance) * 10u32.pow(a as u32 - 1);
                    let c = cache.get(&new_state.get_key()).unwrap_or(&u32::MAX);
                    if c > &new_state.energy {
                        cache.insert(new_state.get_key(), new_state.energy);
                        heap.push(new_state);
                    }
                }
            }
        }
    }
    panic!("not found")
}

#[derive(Clone, Eq, PartialEq)]
struct State<const R: usize> {
    rooms: [u8; R],
    hall: [u8; 7],
    energy: u32,
}

impl<const R: usize> State<R> {
    fn get_key(&self) -> u64 {
        self.rooms
            .iter()
            .chain(&self.hall)
            .fold(0, |acc, &n| acc * 5 + n as u64)
    }
}

impl<const R: usize> Ord for State<R> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.energy.cmp(&self.energy)
    }
}

impl<const R: usize> PartialOrd for State<R> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn distance_row(i0: usize, hall: &[u8]) -> Vec<(usize, u32)> {
    let mut v = vec![];
    let mut i_left = i0 + 1;
    let mut distance = 2;
    'w: while hall.get(i_left) == Some(&0) {
        v.push((i_left, distance));
        if i_left == 0 {
            break 'w;
        }
        i_left -= 1;
        distance += if (1..5).contains(&(i_left as usize)) {
            2
        } else {
            1
        };
    }
    let mut i_right = i0 + 2;
    distance = 2;

    while hall.get(i_right) == Some(&0) {
        v.push((i_right, distance));
        i_right += 1;
        distance += if (2..6).contains(&(i_right as usize)) {
            2
        } else {
            1
        };
    }
    v
}
fn distance_room(i0: usize, room: u8, hall: &[u8]) -> Option<u32> {
    let room = room as f32 + 0.5;
    let mut i0 = i0 as f32;
    let mut dist = 0;
    let sign = (i0 - room).signum();
    if sign.is_sign_positive() {
        while i0 - room > 0.0 {
            i0 -= sign;
            if i0 - room > 0.0 && hall[i0 as usize] != 0 {
                return None;
            }
            dist += if (1..5).contains(&(i0 as usize)) {
                2
            } else {
                1
            };
        }
    } else {
        while room - i0 > 0.0 {
            i0 -= sign;
            if room - i0 > 0.0 && hall[i0 as usize] != 0 {
                return None;
            }
            dist += if (2..6).contains(&(i0 as usize)) {
                2
            } else {
                1
            };
        }
    }

    Some(dist)
}

#[cfg(test)]
mod day23 {

    use super::*;

    const INPUT: &'static str = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), 12521);
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT), 44169);
    }
}
