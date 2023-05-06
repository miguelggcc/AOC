use std::time::Instant;

pub fn day20(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Can't read input file");
    let time = Instant::now();
    //Part 1
    println!("Grove coordinates: {}", do_day20_part1(&input));
    //Part 2
    println!("Part 2, grove coordinates: {}", do_day20_part2(&input));
    println!("{:?}", time.elapsed());
}

fn do_day20_part1(input: &str) -> i64 {
    let v: Vec<i64> = input.lines().map(|l| l.parse::<i64>().unwrap()).collect();
    let indices = (0..v.len()).collect::<Vec<usize>>();

    let mut list = List {
        v: v.clone(),
        i: indices,
        len: v.len() as i64 - 1,
    };
    v.into_iter()
        .enumerate()
        .for_each(|(i, element)| list.mix(element, i));
    let zero = list.v.iter().position(|n| n == &0).unwrap();
    (1000..=3000)
        .step_by(1000)
        .map(|i| list.get(zero + i))
        .sum::<i64>()
}

fn do_day20_part2(input: &str) -> i64 {
    let decryption_key = 811589153;
    let v: Vec<i64> = input
        .lines()
        .map(|l| l.parse::<i64>().unwrap() * decryption_key)
        .collect();
    let indices = (0..v.len()).collect::<Vec<usize>>();

    let mut list = List {
        v: v.clone(),
        i: indices,
        len: v.len() as i64 - 1,
    };
    v.into_iter()
        .enumerate()
        .cycle()
        .take(list.i.len() * 10)
        .for_each(|(i, element)| list.mix(element, i));
    let zero = list.v.iter().position(|n| n == &0).unwrap();
    (1000..=3000)
        .step_by(1000)
        .map(|i| list.get(zero + i))
        .sum::<i64>()
}

struct List {
    v: Vec<i64>,
    i: Vec<usize>,
    len: i64,
}

impl List {
    fn get(&mut self, i: usize) -> i64 {
        *self.v.get(i % self.v.len()).unwrap()
    }
    fn mix(&mut self, element: i64, i: usize) {
        let old_index = self.i.iter().position(|old_i| old_i == &i).unwrap();
        let new_index = (old_index as i64 + element).rem_euclid(self.len) as usize;

        self.i.remove(old_index);
        self.i.insert(new_index, i);

        self.v.remove(old_index);
        self.v.insert(new_index, element);
    }
}

#[cfg(test)]
mod tests {

    use super::do_day20_part1;
    use super::do_day20_part2;
    const INPUT: &'static str = "1
2
-3
3
-2
0
4";

    #[test]
    fn part_1() {
        assert_eq!(do_day20_part1(INPUT), 3)
    }
    #[test]
    fn part_2() {
        assert_eq!(do_day20_part2(INPUT), 1623178306)
    }
}
