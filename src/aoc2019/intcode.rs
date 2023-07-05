#[derive(Clone)]
pub struct IntCode {
    pub p: Vec<isize>,
    i: usize,
    ri: isize,
    input: Vec<isize>,
    pub output: Vec<isize>,
    pub halted: bool,
}

type OnePar = usize;
type TwoPar = (usize, usize);
type ThreePar = (usize, usize, usize);

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
            input: vec![],
            output: vec![],
            halted: false,
        }
    }

    fn next(&mut self) -> usize {
        self.i += 1;
        self.p[self.i - 1] as usize
    }
    fn get_parameter(&mut self, pmode: usize, d: usize) -> OnePar {
        self.i += 1;
        match (pmode / d) % 10 {
            2 => (self.ri + self.p[self.i - 1]) as usize,
            1 => self.i - 1,
            _ => self.p[self.i - 1] as usize,
        }
    }

    fn get_1parameter(&mut self, pmode: usize) -> OnePar {
        self.get_parameter(pmode, 1)
    }

    fn get_2parameters(&mut self, pmode: usize) -> TwoPar {
        (self.get_parameter(pmode, 1), self.get_parameter(pmode, 10))
    }

    fn get_3parameters(&mut self, pmode: usize) -> ThreePar {
        (
            self.get_parameter(pmode, 1),
            self.get_parameter(pmode, 10),
            self.get_parameter(pmode, 100),
        )
    }

    fn get_p(&self, i: usize) -> isize {
        *self.p.get(i).unwrap_or(&0)
    }

    fn get_pmut(&mut self, i: usize) -> &mut isize {
        if i >= self.p.len() {
            self.p.resize(i + 1, 0);
        }
        self.p.get_mut(i).unwrap()
    }

    pub fn execute_input(&mut self, n: isize) {
        self.input.push(n);
        self.execute();
    }

    pub fn execute_inputs(&mut self, n: Vec<isize>) {
        self.input.extend(n.into_iter().rev());
        self.execute();
    }

    pub fn execute_string(&mut self, mut s: String) {
        s.push('\n');
        self.input.extend(s.bytes().rev().map(|n| n as isize));
        self.execute();
    }

    pub fn execute(&mut self) {
        while self.i < self.p.len() && !self.halted {
            let opcode = self.next();
            let pmode = opcode / 100;
            match opcode % 100 {
                1 => self.add(pmode),
                2 => self.mul(pmode),
                3 => {
                    if !self.input(pmode) {
                        break;
                    }
                }
                4 => self.output(pmode),
                5 => self.jump_if_true(pmode),
                6 => self.jump_if_false(pmode),
                7 => self.less_than(pmode),
                8 => self.equal(pmode),
                9 => self.relative_offset(pmode),
                99 => self.halt(),
                e => panic!("uknown instruction {e}"),
            }
        }
    }

    fn add(&mut self, pmode: usize) {
        let (p1, p2, o) = self.get_3parameters(pmode);
        *self.get_pmut(o) = self.get_p(p1) + self.get_p(p2)
    }

    fn mul(&mut self, pmode: usize) {
        let (p1, p2, o) = self.get_3parameters(pmode);
        *self.get_pmut(o) = self.get_p(p1) * self.get_p(p2)
    }

    fn input(&mut self, pmode: usize) -> bool {
        let o = self.get_1parameter(pmode);
        if let Some(last) = self.input.pop() {
            *self.get_pmut(o) = last;
            return true;
        }
        self.i -= 2;
        false
    }

    fn output(&mut self, pmode: usize) {
        let p1 = self.get_1parameter(pmode);
        self.output.push(self.get_p(p1))
    }

    fn jump_if_true(&mut self, pmode: usize) {
        let (p1, p2) = self.get_2parameters(pmode);
        if self.get_p(p1) != 0 {
            self.i = self.get_p(p2) as usize
        }
    }

    fn jump_if_false(&mut self, pmode: usize) {
        let (p1, p2) = self.get_2parameters(pmode);
        if self.get_p(p1) == 0 {
            self.i = self.get_p(p2) as usize;
        }
    }

    fn less_than(&mut self, pmode: usize) {
        let (p1, p2, o) = self.get_3parameters(pmode);

        *self.get_pmut(o) = isize::from(self.get_p(p1) < self.get_p(p2))
    }

    fn equal(&mut self, pmode: usize) {
        let (p1, p2, o) = self.get_3parameters(pmode);
        *self.get_pmut(o) = isize::from(self.get_p(p1) == self.get_p(p2))
    }

    fn relative_offset(&mut self, pmode: usize) {
        let p1 = self.get_1parameter(pmode);
        self.ri += self.get_p(p1)
    }

    fn halt(&mut self) {
        self.halted = true;
    }

    #[allow(dead_code)]
    pub fn get_output_ascii(&mut self) -> String {
        String::from_utf8(self.output.drain(..).map(|c| c as u8).collect()).unwrap()
    }
}
