#[derive(Clone)]
pub struct IntCode {
    pub p: Vec<isize>,
    i: usize,
    ri: isize,
    pub output: Vec<isize>,
    pub halted: bool,
}

type Parameters = (usize, usize);
enum Instruction {
    Add(Parameters, usize),
    Mul(Parameters, usize),
    Input(usize),
    Output(usize),
    JumpIfTrue(Parameters),
    JumpIfFalse(Parameters),
    LessThan(Parameters, usize),
    Equal(Parameters, usize),
    RelativeOffset(usize),
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
            ri: 0,
            output: vec![],
            halted: false,
        }
    }
    fn get_instruction(&mut self) -> Instruction {
        let opcode = self.next();
        let pmode = opcode / 100;
        match opcode % 100 {
            1 => Instruction::Add(self.get_parameters(pmode), self.get_parameter(pmode, 100)),
            2 => Instruction::Mul(self.get_parameters(pmode), self.get_parameter(pmode, 100)),
            3 => Instruction::Input(self.get_parameter(pmode, 1)),
            4 => Instruction::Output(self.get_parameter(pmode, 1)),
            5 => Instruction::JumpIfTrue(self.get_parameters(pmode)),
            6 => Instruction::JumpIfFalse(self.get_parameters(pmode)),
            7 => Instruction::LessThan(self.get_parameters(pmode), self.get_parameter(pmode, 100)),
            8 => Instruction::Equal(self.get_parameters(pmode), self.get_parameter(pmode, 100)),
            9 => Instruction::RelativeOffset(self.get_parameter(pmode, 1)),
            99 => Instruction::Halt,
            e => panic!("uknown instruction {e}"),
        }
    }

    fn next(&mut self) -> usize {
        self.i += 1;
        self.p[self.i - 1] as usize
    }
    fn get_parameter(&mut self, pmode: usize, d: usize) -> usize {
        self.i += 1;
        match (pmode / d) % 10 {
            2 => (self.ri + self.p[self.i - 1]) as usize,
            1 => self.i - 1,
            _ => self.p[self.i - 1] as usize,
        }
    }

    fn get_parameters(&mut self, pmode: usize) -> Parameters {
        (self.get_parameter(pmode, 1), self.get_parameter(pmode, 10))
    }
    fn p(&self, i: usize) -> isize {
        *self.p.get(i).unwrap_or(&0)
    }
    fn pmut(&mut self, i: usize) -> &mut isize {
        if i >= self.p.len() {
            self.p.resize(i + 1, 0);
        }
        self.p.get_mut(i).unwrap()
    }

    pub fn execute(&mut self, mut n: Vec<isize>) {
        while self.i < self.p.len() && !self.halted {
            match self.get_instruction() {
                Instruction::Add((p1, p2), o) => *self.pmut(o) = self.p(p1) + self.p(p2),
                Instruction::Mul((p1, p2), o) => *self.pmut(o) = self.p(p1) * self.p(p2),
                Instruction::Input(o) => {
                    if let Some(last) = n.pop() {
                        *self.pmut(o) = last;
                    } else {
                        self.i -= 2;
                        break;
                    }
                }
                Instruction::Output(p1) => self.output.push(self.p(p1)),
                Instruction::JumpIfTrue((p1, p2)) if self.p(p1) != 0 => {
                    self.i = self.p(p2) as usize
                }
                Instruction::JumpIfFalse((p1, p2)) if self.p(p1) == 0 => {
                    self.i = self.p(p2) as usize;
                }
                Instruction::LessThan((p1, p2), o) => {
                    *self.pmut(o) = isize::from(self.p(p1) < self.p(p2))
                }
                Instruction::Equal((p1, p2), o) => {
                    *self.pmut(o) = isize::from(self.p(p1) == self.p(p2))
                }
                Instruction::RelativeOffset(p1) => self.ri += self.p(p1),
                Instruction::Halt => {
                    self.halted = true;
                    break;
                }
                _ => (),
            }
        }
    }
}
