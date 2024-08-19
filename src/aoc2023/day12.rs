pub fn part1(input: &str) -> impl std::fmt::Display {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(' ').unwrap();
            let springs: Vec<_> = left.trim_end_matches('.').chars().collect();
            let groups: Vec<_> = right.split(',').map(|c| c.parse().unwrap()).collect();
            let mut cache = Cache::new(springs.len(), groups.len());
            get_count(State::default(), &springs, &groups, &mut cache)
        })
        .sum::<u64>()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(' ').unwrap();
            let springs: Vec<_> = [left; 5].join("?").trim_end_matches('.').chars().collect();
            let groups = right
                .split(',')
                .map(|c| c.parse().unwrap())
                .collect::<Vec<_>>()
                .repeat(5);
            let mut cache = Cache::new(springs.len(), groups.len());
            get_count(State::default(), &springs, &groups, &mut cache)
        })
        .sum::<u64>()
}

type State = (usize, usize);
struct Cache {
    cache: Vec<Option<u64>>,
    l: usize,
}

impl Cache {
    fn new(l_springs: usize, l_groups: usize) -> Self {
        Self {
            cache: vec![None; (l_springs + 1) * (l_groups + 1)],
            l: l_springs + 1,
        }
    }
    fn get(&self, s: State) -> Option<u64> {
        self.cache[s.0 + s.1 * self.l]
    }
    fn insert(&mut self, s: State, v: u64) {
        self.cache[s.0 + s.1 * self.l] = Some(v)
    }
}

fn get_count((i, ig): State, springs: &[char], groups: &[usize], cache: &mut Cache) -> u64 {
    if let Some(v) = cache.get((i, ig)) {
        return v;
    }

    if ig == groups.len() {
        //no more groups to check
        return !springs[i..].contains(&'#') as u64;
        //if no more damaged springs, it's possible
    }
    let possible_count = match springs.get(i) {
        Some('.') => get_count((i + 1, ig), springs, groups, cache),
        Some('#') => damaged((i, ig), springs, groups, cache),
        Some('?') => {
            get_count((i + 1, ig), springs, groups, cache)
                + damaged((i, ig), springs, groups, cache)
        }
        _ => 0,
    };
    cache.insert((i, ig), possible_count);
    possible_count
}

fn damaged((i, ig): State, springs: &[char], groups: &[usize], cache: &mut Cache) -> u64 {
    let group = groups[ig];
    if i + group > springs.len() || springs[i..i + group].contains(&'.') {
        //not possible if group is too long or too short
        return 0;
    }
    if springs.len() - i == group {
        // last springs same length as group
        return (ig == groups.len() - 1) as u64;
        // if it's the last group it's possible, otherwise it's not
    }
    if springs[i + group] != '#' {
        //if spring at end of group is a separator ('.' or '?'), move to next group
        return get_count((i + group + 1, ig + 1), springs, groups, cache);
    }
    0
}

#[cfg(test)]
mod day12 {

    use super::*;

    const INPUT: &'static str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), "21");
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT).to_string(), "525152");
    }
}
