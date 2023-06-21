#[derive(Clone)]
pub struct IntCode {
    pub p: Vec<isize>,
    i: usize,
    pub output: Vec<isize>,
    pub halted: bool,
}

enum Instruction {
    Add([usize; 2], usize),
    Mul([usize; 2], usize),
    Input(usize),
    Output(usize),
    JumpIfTrue([usize; 2]),
    JumpIfFalse([usize; 2]),
    LessThan([usize; 2], usize),
    Equal([usize; 2], usize),
    Halt,
}

impl IntCode {
    pub fn new(input: &str) -> Self {
        let p = input
            .split(',')
            .map(|n| n.parse::<isize>().unwrap())
            .collect();
        Self {
            p,
            i: 0,
            output: vec![],
            halted: false,
        }
    }
    fn get_instruction(&mut self) -> Instruction {
        let opcode = self.next();
        let pmode = opcode / 100;
        match opcode % 100 {
            1 => Instruction::Add(self.get_parameters(pmode), self.next()),
            2 => Instruction::Mul(self.get_parameters(pmode), self.next()),
            3 => Instruction::Input(self.next()),
            4 => Instruction::Output(self.get_one_parameter(pmode, 1)),
            5 => Instruction::JumpIfTrue(self.get_parameters(pmode)),
            6 => Instruction::JumpIfFalse(self.get_parameters(pmode)),
            7 => Instruction::LessThan(self.get_parameters(pmode), self.next()),
            8 => Instruction::Equal(self.get_parameters(pmode), self.next()),
            99 => Instruction::Halt,
            e => panic!("uknown instruction {e}"),
        }
    }

    fn next(&mut self) -> usize {
        self.i += 1;
        self.p[self.i - 1] as usize
    }
    fn get_one_parameter(&mut self, pmode: usize, d: usize) -> usize {
        self.i += 1;
        if (pmode / d) % 10 == 1 {
            self.i - 1
        } else {
            self.p[self.i - 1] as usize
        }
    }

    fn get_parameters(&mut self, pmode: usize) -> [usize; 2] {
        [
            self.get_one_parameter(pmode, 1),
            self.get_one_parameter(pmode, 10),
        ]
    }

    pub fn execute(&mut self, mut n: Vec<isize>) {
        while self.i < self.p.len() && !self.halted {
            match self.get_instruction() {
                Instruction::Add([p1, p2], o) => self.p[o] = self.p[p1] + self.p[p2],
                Instruction::Mul([p1, p2], o) => self.p[o] = self.p[p1] * self.p[p2],
                Instruction::Input(o) => {
                    if let Some(last) = n.pop() {
                        self.p[o] = last;
                    } else {
                        self.i -= 2;
                        break;
                    }
                }
                Instruction::Output(o) => self.output.push(self.p[o]),
                Instruction::JumpIfTrue([p1, p2]) if self.p[p1] != 0 => {
                    self.i = self.p[p2] as usize
                }
                Instruction::JumpIfFalse([p1, p2]) if self.p[p1] == 0 => {
                    self.i = self.p[p2] as usize;
                }
                Instruction::LessThan([p1, p2], o) => {
                    self.p[o] = isize::from(self.p[p1] < self.p[p2])
                }
                Instruction::Equal([p1, p2], o) => {
                    self.p[o] = isize::from(self.p[p1] == self.p[p2])
                }
                Instruction::Halt => {
                    self.halted = true;
                    break;
                }
                _ => (),
            }
        }
    }
}

#[cfg(test)]
mod intcode {

    use super::*;

    #[test]
    fn test_day5() {
        let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        let mut computer = IntCode::new(input);
        computer.execute(vec![5]);
        assert_eq!(
            computer
                .output
                .into_iter()
                .map(|n| n.to_string())
                .collect::<String>(),
            "999"
        );
    }
}
