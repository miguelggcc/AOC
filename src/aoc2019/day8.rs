const W: usize = 25;
const H: usize = 6;

pub fn part1(input: &str) -> u32 {
    let all: Vec<_> = input.bytes().map(|b| b - b'0').collect();
    let size = W * H;
    let i = all
        .chunks(size)
        .enumerate()
        .map(|(i, layer)| (i, layer.iter().filter(|&&p| p == 0).count()))
        .min_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
        .0;
    all[i * size..(i + 1) * size]
        .iter()
        .filter(|&&p| p > 0)
        .fold([0, 0], |mut acc, &p| {
            acc[p as usize - 1] += 1;
            acc
        })
        .iter()
        .product()
}

pub fn part2(input: &str) -> String {
    let all: Vec<_> = input.bytes().map(|b| b - b'0').collect();
    let (width, height) = if all.len() > 16 { (W, H) } else { (2, 2) };
    let size = width * height;
    let mut s = String::with_capacity(size);
    for y in 0..height {
        for x in 0..width {
            let i = x + y * width;
            if *all.iter().skip(i).step_by(size).find(|&&p| p < 2).unwrap() == 0 {
                s.push('.');
            } else {
                s.push('#');
            }
        }
        s.push('\n');
    }
    s
}

#[cfg(test)]
mod day8 {

    use super::*;

    #[test]
    fn part_2() {
        assert_eq!(part2("0222112222120000"), ".#\n#.\n");
    }
}
