use std::time::Instant;

pub fn day12(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Can't read input file");
    let time = Instant::now();
    //Part 1
    println!("Number of visible trees: {}", do_day12_part1(&input));
    //Part 2
    //println!("Highest scenic score: {}", do_day12_part2(&input));
    println!("{:?}", time.elapsed());
}

fn do_day12_part1(input: &str) -> u32 {
    let matrix: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let ny = matrix.len();
    let nx = matrix[0].len();
    let matrix: Vec<char> = matrix.into_iter().flatten().collect();
    let start = matrix.iter().position(|c|*c=='S').expect("No start cell found");

    0
}

fn do_day12_part2(input: &str) -> u32 {
    0
}

#[inline(always)]
fn ix(i: usize, j: usize, n: usize) -> usize {
    i + j * n
}

#[cfg(test)]
mod tests {

    use super::do_day12_part1;
    use super::do_day12_part2;

    #[test]
    fn part_1() {
        let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

        assert_eq!(do_day12_part1(input), 31);
        //assert_eq!(do_day12_part2(input), 8)
    }
}
