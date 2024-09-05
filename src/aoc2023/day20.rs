use std::collections::{HashMap, VecDeque};

pub fn part1(input: &str) -> impl std::fmt::Display {
    parse_and_press_button_n_times(input, 1000, false)
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    parse_and_press_button_n_times(input, 4500, true)
}
enum Module {
    Broadcaster,
    FlipFlop(bool),
    Conjuction(u64, u64),
}

fn parse_and_press_button_n_times(input: &str, n: usize, part2: bool) -> usize {
    let mut modules = Vec::with_capacity(64);
    let mut children_map = Vec::with_capacity(64);
    let mut modules_map: HashMap<_, _> = input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            let (m, dest_str) = l.split_once(" -> ").unwrap();
            let (id, module) = match m.split_at(1) {
                ("%", id) => (id, Module::FlipFlop(false)),
                ("&", id) => (id, Module::Conjuction(0, 0)),
                _ => (m, Module::Broadcaster),
            };
            modules.push(module);
            children_map.push(dest_str.split(", "));
            (id, i)
        })
        .collect();

    let start = *modules_map.get("broadcaster").unwrap();
    let mut rx = 0;

    let children = children_map
        .into_iter()
        .enumerate()
        .map(|(i, dest)| {
            dest.map(|d| {
                if d == "rx" {
                    rx = i;
                }
                let children_i = *modules_map.entry(d).or_insert_with(|| {
                    modules.push(Module::Broadcaster);
                    modules.len() - 1
                });
                if let Module::Conjuction(_, ref mut key) = modules[children_i] {
                    *key |= 1 << i;
                };
                children_i
            })
            .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut total_pulses = [n, 0];
    let mut cycles = HashMap::new();
    let mut q = VecDeque::new();
    
    for button in 1..n + 1 {
        q.push_back((start, start, false));
        while let Some((i, old_i, pulse)) = q.pop_front() {
            if let Some(new_pulse) = match modules.get_mut(i).unwrap() {
                Module::Broadcaster => Some(pulse),
                Module::FlipFlop(switch) => {
                    if !pulse {
                        *switch = !*switch;
                        Some(*switch)
                    } else {
                        None
                    }
                }
                Module::Conjuction(mem, key) => {
                    if part2 && i == rx && pulse {
                        cycles.entry(old_i).or_insert(button);
                    }
                    *mem = (*mem & !(1 << old_i)) | ((pulse as u64) << old_i); // set bit at old_i to pulse
                    Some(mem != key)
                }
            } {
                for d in children.get(i).into_iter().flatten() {
                    if !part2 {
                        total_pulses[new_pulse as usize] += 1;
                    }
                    q.push_back((*d, i, new_pulse))
                }
            }
        }
    }
    if part2 {
        return cycles.values().product::<usize>();
    }
    total_pulses.into_iter().product::<usize>()
}

#[cfg(test)]
mod day20 {

    use super::*;

    #[test]
    fn part_1() {
        let input1 = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
        let input2 = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
        assert_eq!(part1(input1).to_string(), "32000000");
        assert_eq!(part1(input2).to_string(), "11687500");
    }
}
