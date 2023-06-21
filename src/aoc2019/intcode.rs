use itertools::*;

#[derive(Clone)]
pub struct IntCode {
    p: Vec<isize>,
    i: usize,
    pub output: Option<isize>,
    pub is_halted: bool,
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
            output: None,
            is_halted: false,
        }
    }
    pub fn execute(&mut self, mut n: Vec<isize>) {
        while self.i < self.p.len() && !self.is_halted {
            let (par0, par1) = [100, 1000]
                .iter()
                .enumerate()
                .map(|(index, d)| {
                    if (self.p[self.i] / d) % 10 == 1 {
                        self.i + index + 1
                    } else {
                        *self.p.get(self.i + index + 1).unwrap_or(&0) as usize
                    }
                })
                .collect_tuple()
                .unwrap();

            let opcode = self.p[self.i] % 100;
            let o = *self.p.get(self.i + 3).unwrap_or(&0) as usize;

            match opcode {
                1 => {
                    self.p[o] = self.p[par0] + self.p[par1];
                    self.i += 4
                }
                2 => {
                    self.p[o] = self.p[par0] * self.p[par1];
                    self.i += 4
                }
                3 => {
                    if let Some(last) = n.pop() {
                        self.p[par0] = last;
                        self.i += 2;
                    } else {
                        break;
                    }
                }
                4 => {
                    self.output = Some(self.p[par0]);
                    self.i += 2;
                }
                5 => {
                    if self.p[par0] != 0 {
                        self.i = self.p[par1] as usize
                    } else {
                        self.i += 3;
                    }
                }
                6 => {
                    if self.p[par0] == 0 {
                        self.i += self.p[par1] as usize;
                    } else {
                        self.i += 3;
                    }
                }
                7 => {
                    self.p[o] = isize::from(self.p[par0] < self.p[par1]);
                    self.i += 4
                }
                8 => {
                    self.p[o] = isize::from(self.p[par0] == self.p[par1]);
                    self.i += 4;
                }
                99 => {
                    self.is_halted = true;
                    break;
                }
                e => panic!("uknown command {e}"),
            };
        }
    }
}
