use std::collections::{HashMap, VecDeque};

pub fn part1(input: &str) -> u32 {
    let nx = input.lines().next().unwrap().len() as i32;
    let mut map: Vec<_> = input.lines().flat_map(|l|l.bytes()).collect();
    let mut portals =  HashMap::new();
    map.iter().enumerate().for_each(|(i,&c)| if c.is_ascii_alphabetic(){
        if let Some(&m_point) = MOVES.iter().find(|&(x,y)| map.get((i as i32 + x + y*nx) as usize) == Some(&b'.')){
            let diff_point =  m_point.0 + m_point.1 * nx;
            let id = if diff_point.is_negative(){
                vec![c, map[i  +  diff_point.unsigned_abs() as usize]]
            } else{
                vec![map[i  -  diff_point as usize], c]
            };
            portals.entry(String::from_utf8(id).unwrap()).or_insert(vec![]).push((i as i32 + diff_point) as usize);  
        }
    });
    let start = portals.remove("AA").unwrap()[0];
    map[start] = b'#';
    let mut q = VecDeque::from([(start, 0)]);

    while let Some((pos,d)) = q.pop_front(){
        for m in MOVES{
            let mut new_pos = (pos as i32 + m.0 + m.1*nx) as usize;
            let c = map[new_pos];
            if c!= b'#'{
                if c.is_ascii_alphabetic(){
                    let diff_point = m.0 + m.1*nx;
                    let id = if diff_point.is_negative(){
                        [map[new_pos  -  diff_point.unsigned_abs() as usize],c, ]
                    } else{
                        [c,map[new_pos  +  diff_point as usize]]
                    };
                    let id = std::str::from_utf8(&id).unwrap();
                    if id=="ZZ"{
                        return d
                    }
                    if let Some(portal_exits) = portals.get(id){
                        map[new_pos] = b'#';
                        new_pos = *portal_exits.iter().find(|&&i|i!=pos).unwrap();

                    } else{
                        continue;
                    }
                }
                map[new_pos] = b'#';
                q.push_back((new_pos,d+1));
            }
        }
    }
    map.chunks(nx as usize).for_each(|r|println!("{:?}",r.into_iter().map(|c|char::from(*c)).collect::<String>()));
    0
}

pub fn part2(input: &str) -> u32 {
    0
}

const MOVES: [(i32,i32);4] = [(0,-1),(-1,0), (1,0), (0,1)];

#[cfg(test)]
mod day20 {

    use super::*;

    const INPUT: &'static str = 
"                   A               
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
        let input = 
"             Z L X W       C                 
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
