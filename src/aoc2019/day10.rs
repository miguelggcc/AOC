use itertools::*;

pub fn part1(input: &str) -> u32 {
    let grid: Vec<_> = input.lines().map(|l|l.chars().map(|c| c=='#').collect::<Vec<_>>()).collect();
    let nx = grid[0].len() as isize;
    let ny = grid.len() as isize;
    let grid: Vec<_> = grid.into_iter().flatten().collect();
    let  quadrant: Vec<_> = (1..ny).cartesian_product(1..nx).filter(|&(x,y)|(gcd(x, y)==1 ) ).collect();
    let mut los = vec![(1,0), (-1,0), (0,1), (0,-1)];
    los.extend([(1,1),(-1,1),(-1,-1),(1,-1)].iter().flat_map(|(dx,dy)|quadrant.iter().map(move |(x,y)|(x*dx,y*dy))));
    (1..nx).cartesian_product(1..ny).filter(|&(x,y)|grid[(x + y*nx) as usize]).map(|pos|{
        los.iter().map(|l|{
            u32::from(find_asteroid(&grid, *l,pos, (nx,ny)).is_some())
        }).sum::<u32>()
    }).max().unwrap()
    }

fn find_asteroid(grid: &[bool], l: (isize,isize),(x,y): (isize,isize), (nx,ny): (isize,isize))->Option<(isize,isize)>{
    for i in 1..{
        let (xp,yp) = (x+l.0*i,y+l.1*i);
        if xp<0 || xp>=nx || yp<0 || yp>=ny{
            return None;
        }
        if grid[(xp + yp * nx ) as usize]{
            return Some((xp,yp));
        }
    }
    unreachable!()
}

fn gcd(a: isize, b: isize)->isize{
    if b == 0 {
       return a;
    }
       return gcd(b, a % b);
    }

pub fn part2(input: &str) -> u32 {
    let grid: Vec<_> = input.lines().map(|l|l.chars().map(|c| c=='#').collect::<Vec<_>>()).collect();
    let nx = grid[0].len() as isize;
    let ny = grid.len() as isize;
    let mut grid: Vec<_> = grid.into_iter().flatten().collect();
    let  mut quadrant: Vec<_> = (1..nx).cartesian_product((1..ny).rev()).filter(|&(x,y)|(gcd(x, y)==1 ) ).collect();
    quadrant.sort_by_key(|&(x,y)|((x as f32).atan2(y as f32)*1000000.0) as i32);
    let mut los: Vec<_> = vec![(0,-1)];
    los.extend(quadrant.iter().map(move |&(x,y)|(x,-1*y)));
    los.push((1,0));
    los.extend(quadrant.iter().copied().rev());
    los.push((0,1));
    los.extend(quadrant.iter().map(move |&(x,y)|(-1*x,y)));
    los.push((-1,0));
    los.extend(quadrant.iter().rev().map(move |&(x,y)|(-1*x,-1*y)));
    let pos = (1..nx).cartesian_product(1..ny).filter(|&(x,y)|grid[(x + y*nx) as usize]).map(|pos|{
        (los.iter().map(|l|{
            u32::from(find_asteroid(&grid, *l,pos, (nx,ny)).is_some())
        }).sum::<u32>(), pos)
    }).max_by_key(|&(t,_)|t).unwrap().1;
    let mut count = 0;

                    for l in los.iter(){

            if let Some((x_as,y_as)) = find_asteroid(&grid, *l,pos, (nx,ny)){
                grid[(x_as + y_as*nx) as usize] = false;
                count+=1;
                if count == 200{
                    return (x_as*100+y_as) as u32
                }
            }
    }
    panic!("no 200th asteroid")
    }


#[cfg(test)]
mod day10 {

    use super::*;

    const INPUT: &'static str = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), 210);
    }
    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT), 802);
    }
}
