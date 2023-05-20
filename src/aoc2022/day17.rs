use std::{
    collections::{hash_map::Entry, HashMap},
    iter::once,
};

pub fn part1(input: &str) -> u64 {
    get_height(input, 2022)
}

pub fn part2(input: &str) -> u64 {
    get_height(input, 1000000000000)
}

const WIDTH: usize = 7;

fn get_height(input: &str, n_of_rocks: u64) -> u64 {
    let mut jets = input
        .chars()
        .map(|c| match c {
            '<' => -1,
            '>' => 1,
            _ => unreachable!(),
        })
        .cycle();

    let mut height = 0;
    let mut map = Map(Vec::with_capacity(input.len() * 5));

    let mut repeated = HashMap::new();
    let mut cycle_height = 0;
    let mut r = 0;
    let mut i_jet = 0;

    while r < (n_of_rocks) {
        //One starting from 0 to when it starts repeating, ignore all the repeated outputs and then the last part that finishes before repeating
        let mut rock = Rock::spawn((r % 5) as u32, height + 2);
        map.add_padding(height, rock.t.height());

        loop {
            let dx = jets.next().unwrap();
            i_jet += 1;
            if rock.can_move_sideways(dx, &map) {
                rock.move_sideways(dx);
            }
            if rock.can_move_downwards(&map) {
                rock.move_downwards();
            } else {
                break;
            }
        }
        height = height.max(rock.y + 1);

        rock.blocks().for_each(|(x, y)| map.toggle(x, y));
        r += 1;

        if height >= 15 {
            let key = map.get_bits(height, r % 5, i_jet % input.len());

            match repeated.entry(key) {
                Entry::Occupied(re) => {
                    let (r0, height0) = re.get();
                    let cycle_r = r - r0;
                    let n_of_cycles = (n_of_rocks - r) / cycle_r;
                    cycle_height += (height - height0) as u64 * n_of_cycles;
                    r += cycle_r * n_of_cycles;
                    repeated.clear();
                }
                Entry::Vacant(va) => {
                    va.insert((r, height));
                }
            }
        }
    }
    height as u64 + cycle_height
}

enum RockType {
    Horizontal,
    Sum,
    L,
    Vertical,
    Square,
}

impl RockType {
    fn height(&self) -> usize {
        match self {
            RockType::Horizontal => 1,
            RockType::Sum => 3,
            RockType::L => 3,
            RockType::Vertical => 4,
            RockType::Square => 2,
        }
    }
    fn width(&self) -> usize {
        match self {
            RockType::Horizontal => 4,
            RockType::Sum => 3,
            RockType::L => 3,
            RockType::Vertical => 1,
            RockType::Square => 2,
        }
    }
    #[inline(always)]
    fn blocks(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
        let get_blocks = move |(dx, dy): &(isize, isize)| {
            ((x as isize + dx) as usize, (y as isize + dy) as usize)
        };
        match self {
            RockType::Horizontal => BLOCKS_HORIZONTAL.iter().map(get_blocks),
            RockType::Sum => BLOCKS_SUM.iter().map(get_blocks),
            RockType::L => BLOCKS_L.iter().map(get_blocks),
            RockType::Vertical => BLOCKS_VERTICAL.iter().map(get_blocks),
            RockType::Square => BLOCKS_SQUARE.iter().map(get_blocks),
        }
    }
}
type Point = (isize, isize);
const BLOCKS_HORIZONTAL: [Point; 4] = [(0, 0), (1, 0), (2, 0), (3, 0)];
const BLOCKS_SUM: [Point; 5] = [(1, 0), (0, -1), (1, -1), (2, -1), (1, -2)];
const BLOCKS_L: [Point; 5] = [(2, 0), (2, -1), (0, -2), (1, -2), (2, -2)];
const BLOCKS_VERTICAL: [Point; 4] = [(0, 0), (0, -1), (0, -2), (0, -3)];
const BLOCKS_SQUARE: [Point; 4] = [(0, 0), (1, 0), (0, -1), (1, -1)];

struct Rock {
    t: RockType,
    x: usize,
    y: usize,
}

impl Rock {
    fn spawn(t: u32, height: usize) -> Self {
        let ty = match t {
            0 => RockType::Horizontal,
            1 => RockType::Sum,
            2 => RockType::L,
            3 => RockType::Vertical,
            4 => RockType::Square,
            _ => unreachable!(),
        };
        let extra_height = ty.height();
        Self {
            t: ty,
            x: 2,
            y: height + extra_height,
        }
    }
    fn can_move_sideways(&self, dx: isize, map: &Map) -> bool {
        let wall_check = if dx > 0 {
            self.x + self.t.width() + dx as usize > WIDTH
        } else {
            self.x < dx.unsigned_abs()
        };
        if wall_check {
            false
        } else {
            !self
                .t
                .blocks((self.x as isize + dx) as usize, self.y)
                .any(|(x, y)| map.is_on(x, y))
        }
    }
    fn move_sideways(&mut self, dx: isize) {
        if dx > 0 {
            self.x += dx as usize;
        } else {
            self.x -= dx.unsigned_abs();
        }
    }

    fn can_move_downwards(&self, map: &Map) -> bool {
        if self.y < self.t.height() {
            false
        } else {
            !self
                .t
                .blocks(self.x, self.y - 1)
                .any(|(x, y)| map.is_on(x, y))
        }
    }

    fn move_downwards(&mut self) {
        self.y -= 1;
    }
    fn blocks(&self) -> impl Iterator<Item = (usize, usize)> {
        self.t.blocks(self.x, self.y)
    }
}

pub struct Map(Vec<u8>);

impl Map {
    fn toggle(&mut self, x: usize, y: usize) {
        self.0[y] ^= 1u8 << x
    }
    fn is_on(&self, x: usize, y: usize) -> bool {
        self.0[y] & 1u8 << x != 0
    }
    fn add_padding(&mut self, height: usize, rock_height: usize) {
        let current_height = self.0.len();
        let needed_height = (height + rock_height + 3).saturating_sub(current_height);
        self.0.extend(once(0).cycle().take(needed_height));
    }
    #[inline(always)]
    fn get_bits(&self, height: usize, rock_type: u64, i_jet: usize) -> u128 {
        (self.0[height - 15..height]
            .iter()
            .enumerate()
            .fold(0, |acc, (i, row)| acc | ((*row as u128) << (7 * i))))
            | ((i_jet as u128) << 105)
            | ((rock_type as u128) << 121)
    }
    #[allow(dead_code)]
    fn display(&self) {
        for j in (0..self.0.len()).rev() {
            let mut line = String::new();
            for i in 0..WIDTH {
                if self.is_on(i, j) {
                    line.push('#');
                } else {
                    line.push('.');
                }
            }
            println!("{line}");
        }
        println!("\n");
    }
}

#[cfg(test)]
mod tests {
    use super::part1;
    use super::part2;
    const INPUT: &'static str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), 3068);
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT), 1514285714288);
    }
}
