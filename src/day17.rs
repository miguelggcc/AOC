use std::{
    collections::{hash_map::Entry, HashMap, HashSet, VecDeque},
    time::Instant,
};

const WIDTH: usize = 7;
pub fn day17(input_path: &str) {
    let input = std::fs::read_to_string(input_path).expect("Can't find input file");
    //Part 1
    let time = Instant::now();
    println!("First marker after character {}", do_day17_part1(&input));
    //Part 2
    /*println!(
        "Part2: Height {}",
        do_day17_part2(&input)
    );    */
    println!("{:?}", time.elapsed());
}

fn do_day17_part1(input: &str) -> u64 {
    get_height(input, 74)
}

fn do_day17_part2(input: &str) -> u64 {
    let jet_length = input.len();
    let rock_length = 5;
    let big_number = 2022;
    let mult = rock_length * jet_length as u64 - 1;
    let division = big_number / (mult);
    let module = (big_number % (mult)) as u32;
    dbg!(division, module, mult);

    get_height(input, 0)
}

fn get_height(input: &str, n_of_rocks: u64) -> u64 {
    let mut jets = input
        .chars()
        .map(|c| match c {
            '<' => Jet::Left,
            '>' => Jet::Right,
            _ => unreachable!(),
        })
        .cycle();

    let mut height = 0;
    let mut map = Map(vec![]);

    let mut repeated = HashMap::new();
    let mut cycle_height = 0;
    let mut r = 0;

    while r < (n_of_rocks) {
        //One starting from 0 to when it starts repeating, ignore all the repeated outputs and then the last part that finishes before repeating
        let mut rock = Rock::spawn((r % 5) as u32, height + 2);
        map.add_padding(height, rock.t.height());

        loop {
            let dx = jets.next().unwrap() as isize;
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

        rock.blocks()
            .into_iter()
            .for_each(|(x, y)| map.toggle(x, y));
        r += 1;

        if height > 7 {
            let key = u64::from_be_bytes(map.0[height - 8..height].try_into().unwrap());
            //dbg!(key);
            if r == 54 {
                map.display();
            }

            match repeated.entry(key) {
                Entry::Occupied(re) => {
                    dbg!(key, re.key());
                    let (r0, height0) = re.get();
                    let cycle_r = r - r0;
                    if cycle_r > 5 {
                        let n_of_cycles = (n_of_rocks - r) / cycle_r;
                        dbg!(r, r0, n_of_cycles);
                        cycle_height += (height - height0) as u64 * n_of_cycles;
                        r += cycle_r * n_of_cycles;
                        repeated.clear();
                    }
                }
                Entry::Vacant(va) => {
                    va.insert((r, height));
                }
            }
        }
    }
    map.display();
    height as u64 + cycle_height
}

enum Jet {
    Left = -1,
    Right = 1,
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
    fn blocks(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        match self {
            RockType::Horizontal => vec![(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],
            RockType::Sum => vec![
                (x + 1, y),
                (x, y - 1),
                (x + 1, y - 1),
                (x + 2, y - 1),
                (x + 1, y - 2),
            ],
            RockType::L => vec![
                (x + 2, y),
                (x + 2, y - 1),
                (x, y - 2),
                (x + 1, y - 2),
                (x + 2, y - 2),
            ],
            RockType::Vertical => vec![(x, y), (x, y - 1), (x, y - 2), (x, y - 3)],
            RockType::Square => vec![(x, y), (x + 1, y), (x, y - 1), (x + 1, y - 1)],
        }
    }
}

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
            self.x < dx.abs() as usize
        };
        if wall_check {
            false
        } else {
            !self
                .t
                .blocks((self.x as isize + dx) as usize, self.y)
                .into_iter()
                .any(|(x, y)| map.is_on(x, y))
        }
    }
    fn move_sideways(&mut self, dx: isize) {
        if dx > 0 {
            self.x += dx as usize;
        } else {
            self.x -= dx.abs() as usize;
        }
    }

    fn can_move_downwards(&self, map: &Map) -> bool {
        let (_, wall_check) = self.y.overflowing_sub(self.t.height());
        if wall_check {
            false
        } else {
            !self
                .t
                .blocks(self.x, self.y - 1)
                .into_iter()
                .any(|(x, y)| map.is_on(x, y))
        }
    }

    fn move_downwards(&mut self) {
        self.y -= 1;
    }
    fn blocks(&self) -> Vec<(usize, usize)> {
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
        self.0.extend(vec![0; needed_height]);
    }
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

/*#[cfg(test)]
mod tests {
    use super::do_day17_part1;
    use super::do_day17_part2;

    #[test]
    fn part_1() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

        assert_eq!(do_day17_part1(input), 3227);
    }
}
*/
