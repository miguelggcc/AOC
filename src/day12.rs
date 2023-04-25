use std::{
    collections::{HashMap, HashSet, VecDeque},
    time::Instant,
};

pub fn day12(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Can't read input file");
    let time = Instant::now();
    //Part 1
    println!("Shortest path: {}", do_day12_part1(&input));
    //Part 2
    println!("Part 2 shortest path: {}", do_day12_part2(&input));
    println!("{:?}", time.elapsed());
}

fn do_day12_part1(input: &str) -> u32 {
    let matrix_data: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let ny = matrix_data.len();
    let nx = matrix_data[0].len();
    let mut start = 0;
    let mut end = 0;
    let matrix: Vec<u8> = matrix_data
        .into_iter()
        .flatten()
        .enumerate()
        .map(|(index, c)| match c {
            'S' => {
                start = index;
                0
            }
            'E' => {
                end = index;
                b'z' - b'a'
            }
            _ => c as u8 - b'a',
        })
        .collect();

    let mut visited = HashSet::new();
    visited.insert(start);
    let mut queue = VecDeque::new();
    queue.push_back(start);
    let mut distance = HashMap::new();
    distance.insert(start, 0);

    while let Some(index) = queue.pop_front() {
        let dist = *distance.get_mut(&index).unwrap();
        for neighbour_index in get_neighbours_indices(index, nx, ny) {
            if !visited.contains(&neighbour_index) && matrix[index] + 1 >= matrix[neighbour_index] {
                distance.insert(neighbour_index, dist + 1);
                if neighbour_index == end {
                    visited.insert(end);
                    return dist + 1;
                }
                visited.insert(neighbour_index);
                queue.push_back(neighbour_index);
            }
        }
    }

    panic!("Path not found")
}

fn do_day12_part2(input: &str) -> u32 {
    let matrix_data: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let ny = matrix_data.len();
    let nx = matrix_data[0].len();
    let mut start = 0;
    let matrix: Vec<u8> = matrix_data
        .into_iter()
        .flatten()
        .enumerate()
        .map(|(index, c)| match c {
            'S' => 0,
            'E' => {
                start = index;
                b'z' - b'a'
            }
            _ => c as u8 - b'a',
        })
        .collect();

    let mut visited = HashSet::new();
    visited.insert(start);
    let mut queue = VecDeque::new();
    queue.push_back(start);
    let mut distance = HashMap::new();
    distance.insert(start, 0);

    while let Some(index) = queue.pop_front() {
        let dist = *distance.get_mut(&index).unwrap();
        for neighbour_index in get_neighbours_indices(index, nx, ny) {
            if !visited.contains(&neighbour_index) && matrix[index] <= matrix[neighbour_index] + 1 {
                distance.insert(neighbour_index, dist + 1);
                visited.insert(neighbour_index);
                if matrix[neighbour_index] == 0 {
                    return dist + 1;
                }
                queue.push_back(neighbour_index);
            }
        }
    }
    panic!("Path not found")
}

fn get_neighbours_indices(index: usize, nx: usize, ny: usize) -> Vec<usize> {
    let mut indices = Vec::with_capacity(4);
    let i = index % nx;
    let j = index / nx;
    if i > 0 {
        indices.push(ix(i - 1, j, nx))
    }
    if i < nx - 1 {
        indices.push(ix(i + 1, j, nx))
    }
    if j > 0 {
        indices.push(ix(i, j - 1, nx))
    }
    if j < ny - 1 {
        indices.push(ix(i, j + 1, nx))
    }
    indices
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
        assert_eq!(do_day12_part2(input), 29)
    }
}
