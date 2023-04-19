use std::time::Instant;

pub fn day8(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Can't read input file");
    let time = Instant::now();
    //Part 1
    println!("Number of visible trees: {}", do_day8_part1(&input));
    println!("{:?}", time.elapsed());
    //Part 2
    //println!("Part2: {}", do_day8_part2(&input));
}

fn do_day8_part1(input: &str) -> u32 {
    let matrix: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let ny = matrix.len();
    let nx = matrix[0].len();
    let matrix: Vec<u32> = matrix.into_iter().flatten().collect();
    let mut left_max = vec![0; ny * nx];
    let mut right_max = left_max.clone();
    let mut up_max = left_max.clone();
    let mut down_max = left_max.clone();

    for j in 1..ny - 1 {
        let mut max = 0;
        for i in 0..nx - 1 {
            max = max.max(matrix[ix(i, j, nx)]);
            left_max[ix(i + 1, j, nx)] = max;
        }
        let mut max = 0;
        for i in (1..nx).rev() {
            max = max.max(matrix[ix(i, j, nx)]);
            right_max[ix(i - 1, j, nx)] = max;
        }
    }

    for i in 1..nx - 1 {
        let mut max = 0;
        for j in 0..ny - 1 {
            max = max.max(matrix[ix(i, j, nx)]);
            up_max[ix(i, j + 1, nx)] = max;
        }
        let mut max = 0;
        for j in (1..ny).rev() {
            max = max.max(matrix[ix(i, j, nx)]);
            down_max[ix(i, j - 1, nx)] = max;
        }
    }

    /*matrix
        .iter()
        .zip(left_max)
        .zip(right_max)
        .zip(up_max)
        .zip(down_max)
        .filter(|((((n, nl), nr), nu), nd)| &nl < n || &nr < n || &nu < n || &nd < n)
        .count() as u32*/
    let mut total = (nx*2 + (ny-2)*2) as u32;
    for j in 1..ny-1{
        for i in 1..nx-1{
            let n = matrix[ix(i,j,nx)];

            if left_max[ix(i,j,nx)]<n || right_max[ix(i,j,nx)]<n || up_max[ix(i,j,nx)]<n || down_max[ix(i,j,nx)]<n{
                total+=1;
            }
        }
    }
    total
}

#[inline(always)]
fn ix(i: usize, j: usize, n: usize) -> usize {
    i + j * n
}

#[cfg(test)]
mod tests {
    use super::do_day8_part1;

    #[test]
    fn part_1() {
        let input = "30373
25512
65332
33549
35390";

        assert_eq!(do_day8_part1(input), 21);
    }
}
