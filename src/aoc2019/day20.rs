use std::collections::{HashMap, VecDeque};

pub fn part1(input: &str) -> u32 {
    let (mut map, nx, _, mut portals) = parse_map(input);

    let aa = portals.remove("AA").unwrap()[0];
    map[aa] = b'#';
    let mut q = VecDeque::from([(aa, 0)]);

    while let Some((pos, d)) = q.pop_front() {
        for m in MOVES {
            let mut new_pos = (pos as i32 + m.0 + m.1 * nx) as usize;
            let c = map[new_pos];
            if c != b'#' {
                if c.is_ascii_alphabetic() {
                    let diff_point = m.0 + m.1 * nx;
                    let id = if diff_point.is_negative() {
                        [map[new_pos - diff_point.unsigned_abs() as usize], c]
                    } else {
                        [c, map[new_pos + diff_point as usize]]
                    };
                    let id = std::str::from_utf8(&id).unwrap();

                    if id == "ZZ" {
                        return d;
                    }
                    if let Some(portal_exits) = portals.get(id) {
                        map[new_pos] = b'#';
                        new_pos = *portal_exits.iter().find(|&&i| i != pos).unwrap();
                    } else {
                        continue;
                    }
                }
                map[new_pos] = b'#';
                q.push_back((new_pos, d + 1));
            }
        }
    }
    panic!("path not found")
}

pub fn part2(input: &str) -> u32 {
    let (map, nx, ny, portals) = parse_map(input);

    let mut nodes = Vec::with_capacity(portals.len() * 2 - 2);
    let mut indices = HashMap::new();
    let mut index = 0;
    let aa = portals.get("AA").unwrap()[0];

    portals.into_iter().for_each(|(id, mut pos)| {
        pos.sort_by_key(|&p| {
            let (px, py) = (p as i32 % nx, p as i32 / nx);
            px > 2 && px < nx - 3 && py > 2 && py < ny - 3
        });
        for (i, p) in pos.into_iter().enumerate() {
            indices.insert(p, index);
            index += 1;
            nodes.push(Portal {
                id: id.clone(),
                inside: i == 1,
                pos: p,
                paths: vec![],
            });
        }
    });

    for node in &mut nodes {
        node.floodfill(aa, map.clone(), nx, &indices);
    }

    let mut q = VecDeque::from([(*indices.get(&aa).unwrap(), 0, 0)]);

    while let Some((parent_index, total_d, level)) = q.pop_front() {
        for &(index, d) in nodes[parent_index].paths.iter() {
            if nodes[index].id == "ZZ" {
                if level == 0 {
                    return total_d + d - 1;
                }
            } else if nodes[index].inside {
                q.push_back((index - 1, total_d + d, level + 1));
            } else if level > 0 {
                q.push_back((index + 1, total_d + d, level - 1));
            }
        }
    }
    panic!("path not found")
}

struct Portal {
    id: String,
    inside: bool,
    pos: usize,
    paths: Vec<(usize, u32)>,
}

impl Portal {
    fn floodfill(&mut self, aa: usize, mut map: Vec<u8>, nx: i32, indices: &HashMap<usize, usize>) {
        map[self.pos] = b'#';
        let mut q = VecDeque::from([(self.pos, 0)]);
        while let Some((pos, d)) = q.pop_front() {
            for m in MOVES {
                let new_pos = (pos as i32 + m.0 + m.1 * nx) as usize;
                let c = map[new_pos];
                if c != b'#' {
                    if c.is_ascii_alphabetic() {
                        if d > 0 && pos != aa {
                            self.paths.push((*indices.get(&pos).unwrap(), d + 1));
                        }
                        continue;
                    }
                    map[new_pos] = b'#';
                    q.push_back((new_pos, d + 1));
                }
            }
        }
    }
}

fn parse_map(input: &str) -> (Vec<u8>, i32, i32, HashMap<String, Vec<usize>>) {
    let nx = input.lines().next().unwrap().len() as i32;
    let map: Vec<_> = input.lines().flat_map(|l| l.bytes()).collect();
    let ny = map.len() as i32 / nx;

    let mut portals = HashMap::new();
    map.iter().enumerate().for_each(|(i, &c)| {
        if c.is_ascii_alphabetic() {
            if let Some(&m_point) = MOVES
                .iter()
                .find(|&(x, y)| map.get((i as i32 + x + y * nx) as usize) == Some(&b'.'))
            {
                let d_2_point = m_point.0 + m_point.1 * nx;
                let id = if d_2_point.is_negative() {
                    vec![c, map[i + d_2_point.unsigned_abs() as usize]]
                } else {
                    vec![map[i - d_2_point as usize], c]
                };

                portals
                    .entry(String::from_utf8(id).unwrap())
                    .or_insert(vec![])
                    .push((i as i32 + d_2_point) as usize);
            }
        }
    });
    (map, nx, ny, portals)
}

const MOVES: [(i32, i32); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];

#[cfg(test)]
mod day20 {

    use super::*;

    const INPUT: &'static str = "                   A               
                   A               
  #################.#############  
  #.#...#...................#.#.#  
  #.#.#.###.###.###.#########.#.#  
  #.#.#.......#...#.....#.#.#...#  
  #.#########.###.#####.#.#.###.#  
  #.............#.#.....#.......#  
  ###.###########.###.#####.#.#.#  
  #.....#        A   C    #.#.#.#  
  #######        S   P    #####.#  
  #.#...#                 #......VT
  #.#.#.#                 #.#####  
  #...#.#               YN....#.#  
  #.###.#                 #####.#  
DI....#.#                 #.....#  
  #####.#                 #.###.#  
ZZ......#               QG....#..AS
  ###.###                 #######  
JO..#.#.#                 #.....#  
  #.#.#.#                 ###.#.#  
  #...#..DI             BU....#..LF
  #####.#                 #.#####  
YN......#               VT..#....QG
  #.###.#                 #.###.#  
  #.#...#                 #.....#  
  ###.###    J L     J    #.#.###  
  #.....#    O F     P    #.#...#  
  #.###.#####.#.#####.#####.###.#  
  #...#.#.#...#.....#.....#.#...#  
  #.#####.###.###.#.#.#########.#  
  #...#.#.....#...#.#.#.#.....#.#  
  #.###.#####.###.###.#.#.#######  
  #.#.........#...#.............#  
  #########.###.###.#############  
           B   J   C               
           U   P   P               ";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), 58);
    }
    #[test]
    fn part_2() {
        let input = "             Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     ";
        assert_eq!(part2(input), 396);
    }
}
