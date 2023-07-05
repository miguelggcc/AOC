use super::intcode::IntCode;

pub fn part1(input: &str) -> usize {
    let mut ic = IntCode::new(input);
    ic.execute();
    let nx = ic.output.iter().position(|&c| c == 10).unwrap();
    let map: Vec<_> = ic
        .output
        .drain(..)
        .filter(|&c| c != 10)
        .map(|c| c == 35)
        .collect();

    map[nx..map.len() - nx]
        .iter()
        .enumerate()
        .filter(|&(i, c)| {
            *c && i % nx > 1
                && i % nx < nx - 1
                && map[i - 1]
                && map[i + 1]
                && map[i - nx]
                && map[i + nx]
        })
        .map(|(i, _)| (i % nx) * (i / nx))
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let mut ic = IntCode::new(input);

    let mut copy = ic.clone();
    copy.execute();
    let nx = copy.output.iter().position(|&c| c == 10).unwrap() as isize;
    let mut pos = copy
        .output
        .iter()
        .filter(|&&c| c != 10)
        .position(|&c| c == b'^' as isize)
        .map(|p| (p as isize % nx, p as isize / nx))
        .unwrap();
    let map: Vec<_> = copy
        .output
        .drain(..)
        .filter(|&c| c != 10)
        .map(|c| c == 35)
        .collect();

    let mut dir = 0;
    let mut inst: Vec<(u8, u8)> = vec![];

    loop {
        let m = MOVES[dir];
        let (x, y) = (pos.0 + m.0, pos.1 + m.1);

        if x >= 0 && x < nx && map.get((x + y * nx) as usize) == Some(&true) {
            pos = (x, y);
            inst.last_mut().unwrap().1 += 1;
        } else if let Some(t_index) = TURNS[dir].iter().position(|&t| {
            let (x, y) = (pos.0 + t.0, pos.1 + t.1);
            x >= 0 && x < nx && map.get((x + y * nx) as usize) == Some(&true)
        }) {
            let t = TURNS[dir][t_index];
            pos = (pos.0 + t.0, pos.1 + t.1);
            inst.push(([b'L', b'R'][t_index], 1));
            dir = INDEX[dir][t_index];
        } else {
            break;
        }
    }
    //let inst: Vec<u8> = inst.into_iter().flat_map(|(t,d)|{let mut v = vec![t]; v.extend(d.to_string().bytes()); v.push(b','); v}).collect();
    //dbg!(std::str::from_utf8(&inst).unwrap());

    let complete = "A,B,B,A,B,C,A,C,B,C";
    let a = "L,4,L,6,L,8,L,12";
    let b = "L,8,R,12,L,12";
    let c = "R,12,L,6,L,6,L,8";
    let path = [complete, a, b, c, "n"].join("\n");
    ic.p[0] = 2;
    ic.execute_string(path);
    ic.output.pop().unwrap() as u32
}

const MOVES: [(isize, isize); 4] = [(0, -1), (0, 1), (1, 0), (-1, 0)];
const TURNS: [[(isize, isize); 2]; 4] = [
    [(-1, 0), (1, 0)],
    [(1, 0), (-1, 0)],
    [(0, -1), (0, 1)],
    [(0, 1), (0, -1)],
];
const INDEX: [[usize; 2]; 4] = [[3, 2], [2, 3], [0, 1], [1, 0]];
