use std::{collections::HashSet, ops::AddAssign};

pub fn part1(input: &str) -> u32 {
    let mut head = Vec2::new(0, 0);
    let mut tail = Vec2::new(0, 0);
    let mut visited = HashSet::new();

    input.lines().for_each(|l| {
        let (dir, d) = match l.split_whitespace().collect::<Vec<_>>()[..] {
            ["R", d] => (Vec2::new(1, 0), d),
            ["L", d] => (Vec2::new(-1, 0), d),
            ["D", d] => (Vec2::new(0, -1), d),
            ["U", d] => (Vec2::new(0, 1), d),
            _ => unreachable!(),
        };

        for _ in 0..d.parse::<usize>().unwrap() {
            head.move_in_dir(&dir);
            follow_head_mut(&mut tail, &head);
            visited.insert(tail.to_array());
        }
    });
    visited.len() as u32
}

pub fn part2(input: &str) -> u32 {
    let mut head = Vec2::new(0, 0);
    let mut rope = vec![Vec2::new(0, 0); 9];
    let mut visited = HashSet::new();

    input.lines().for_each(|l| {
        let (dir, d) = match l.split_whitespace().collect::<Vec<_>>()[..] {
            ["R", d] => (Vec2::new(1, 0), d),
            ["L", d] => (Vec2::new(-1, 0), d),
            ["D", d] => (Vec2::new(0, -1), d),
            ["U", d] => (Vec2::new(0, 1), d),
            _ => unreachable!(),
        };
        for _ in 0..d.parse::<usize>().unwrap() {
            head.move_in_dir(&dir);
            follow_head_mut(&mut rope[0], &head);
            for i in 1..rope.len() {
                rope[i] = follow_head(&rope[i], &rope[i - 1])
            }
            visited.insert(rope.last().unwrap().to_array());
        }
    });
    visited.len() as u32
}

fn follow_head_mut(tail: &mut Vec2<i32>, head: &Vec2<i32>) {
    let dx = head.x - tail.x;
    let dy = head.y - tail.y;

    if dx.abs() + dy.abs() > 2 {
        tail.x += dx.signum();
        tail.y += dy.signum();
    } else {
        if dx.abs() > 1 {
            tail.x += dx.signum();
        }
        if dy.abs() > 1 {
            tail.y += dy.signum();
        }
    }
}
fn follow_head(tail: &Vec2<i32>, head: &Vec2<i32>) -> Vec2<i32> {
    let dx = head.x - tail.x;
    let dy = head.y - tail.y;
    let mut output = Vec2::new(tail.x, tail.y);

    if dx.abs().max(dy.abs()) > 1 {
        output.x += dx.signum();
        output.y += dy.signum();
    }
    output
}

#[derive(Clone, Debug)]
struct Vec2<T> {
    x: T,
    y: T,
}

impl<T> Vec2<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
    fn to_array(&self) -> [T; 2]
    where
        T: Copy,
    {
        [self.x, self.y]
    }
    fn move_in_dir(&mut self, dir: &Vec2<T>)
    where
        T: AddAssign + Copy,
    {
        self.x += dir.x;
        self.y += dir.y;
    }
}

#[cfg(test)]
mod tests {

    use super::part1;

    #[test]
    fn part_1() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

        assert_eq!(part1(input), 13);
    }
    #[test]
    fn part_2() {
        use super::part2;

        let input = "R 5
        U 8
        L 8
        D 3
        R 17
        D 10
        L 25
        U 20";
        assert_eq!(part2(input), 36)
    }
}
