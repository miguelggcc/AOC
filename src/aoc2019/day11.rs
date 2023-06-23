use super::intcode::IntCode;
use std::collections::HashMap;

pub fn part1(input: &str) -> usize {
    let computer = IntCode::new(input);
    let mut map = HashMap::new();
    let mut robot = Robot {
        pos: (0, 0),
        dir: (0, 1),
    };
    robot.execute_program(computer, &mut map);
    map.len()
}

pub fn part2(input: &str) -> String {
    let computer = IntCode::new(input);
    let mut map = HashMap::new();
    map.insert((0, 0), 1);
    let mut robot = Robot {
        pos: (0, 0),
        dir: (0, 1),
    };

    robot.execute_program(computer, &mut map);

    let (xmin, xmax, ymin, ymax) = map.iter().fold(
        (i8::MAX, i8::MIN, i8::MAX, i8::MIN),
        |(xmin, xmax, ymin, ymax), (p, _)| {
            (xmin.min(p.0), xmax.max(p.0), ymin.min(p.1), ymax.max(p.1))
        },
    );

    let mut s = String::with_capacity(((xmax - xmin + 2) * (ymax - ymin + 1)) as usize);
    for y in (ymin..ymax + 1).rev() {
        for x in xmin..xmax + 1 {
            if map.get(&(x, y)) == Some(&1) {
                s.push('#');
            } else {
                s.push('.');
            }
        }
        s.push('\n');
    }
    s
}
struct Robot {
    pos: (i8, i8),
    dir: (i8, i8),
}

impl Robot {
    fn forward(&mut self) {
        self.pos = (self.pos.0 + self.dir.0, self.pos.1 + self.dir.1)
    }
    fn rotate_ccw(&mut self) {
        std::mem::swap(&mut self.dir.0, &mut self.dir.1);
        self.dir.0 *= -1;
    }
    fn rotate_cw(&mut self) {
        std::mem::swap(&mut self.dir.0, &mut self.dir.1);
        self.dir.1 *= -1;
    }
    fn execute_program(&mut self, mut computer: IntCode, map: &mut HashMap<(i8, i8), isize>) {
        while !computer.halted {
            computer.execute_input(*map.get(&self.pos).unwrap_or(&0));

            if computer.output.pop().unwrap() == 0 {
                self.rotate_ccw();
            } else {
                self.rotate_cw();
            }
            map.insert(self.pos, computer.output.pop().unwrap());
            self.forward();
        }
    }
}
