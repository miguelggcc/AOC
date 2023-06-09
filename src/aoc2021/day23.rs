use std::{collections::{VecDeque, BinaryHeap}, cmp::Ordering};

pub fn part1(input: &str) -> u32 {
    let mut pos = [vec![], vec![],vec![],vec![],vec![0;7]];
    input.lines().rev().skip(1).take(2).enumerate().for_each(|(j,l)|l.trim().split('#').filter(|s|!s.is_empty()).enumerate().for_each(|(i,a)|pos[i].push(a.chars().next().unwrap() as u8 -b'A'+1)));
    let mut heap = BinaryHeap::from([State{pos, energy: 0}]);
    't: while let Some(state) = heap.pop(){
        if state.pos.iter().take(4).enumerate().all(|(i,room)|room.len()==2&&room.iter().all(|&ar|ar==i as u8+1)){
            //if state.energy == 15405{println!("{:?}",row); break 't;}
            return state.energy;
        }
        for (i,a) in state.pos[4].iter().enumerate().filter(|(_,a)|**a!=0){
            if state.pos[*a as usize-1].is_empty() || (state.pos[*a as usize-1].len()==1 && state.pos[*a as usize-1][0]==*a){
                let a = state.pos[4][i];
               if  let Some(distance) = distance_room(i, a, &state.pos[4]){
                let mut new_pos = state.pos.clone();
                new_pos[4][i] = 0;
                new_pos[a as usize -1].push(a);  
                //let mut new_row = row.clone();
                //new_row.push(new_pos.clone().into_iter().flatten().collect::<Vec<_>>());
                //new_row.push(vec![1-state[a as usize -1].len() as u8 +distance as u8]);
                heap.push(State{pos: new_pos, energy: state.energy+ (1-state.pos[a as usize -1].len() as u32+distance)*10u32.pow(a as u32-1)});
            }
        }
        }
        for (i,room) in state.pos.iter().take(4).enumerate(){
            if !room.is_empty() && room.iter().any(|&a|a!=i as u8+1){
                let possible_pos = distance_row(i, &state.pos[4]);
                for (pos, distance) in possible_pos{
                    let mut new_pos = state.pos.clone();
                    let a = new_pos[i].pop().unwrap();
                    new_pos[4][pos] = a;
                    //let mut new_row = row.clone();
                    //new_row.push(new_pos.clone().into_iter().flatten().collect::<Vec<_>>());
                    //new_row.push(vec![2-room.len() as u8,distance as u8]);
                   heap.push(State { pos: new_pos, energy: state.energy+ (2-room.len() as u32+distance)*10u32.pow(a as u32-1 )});
                }
            }
        }

    }
    panic!("not found")
}

pub fn part2(_input: &str) -> String {
    String::from("Not implemented")
}

#[derive(Eq, PartialEq)]
struct State {
    pos:[Vec<u8>;5],
    energy: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .energy
            .cmp(&self.energy)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn distance_row(i0 : usize, hall: &[u8])->Vec<(usize,u32)>{
    let mut v = vec![];
    let mut i_left = i0+1;
    let mut distance = 2;
    'w: while hall.get(i_left) == Some(&0){
        v.push((i_left,distance));
        if i_left==0{
            break 'w;
        }
        i_left-=1;
        distance+=if (1..5).contains(&(i_left as usize)){2}else{1};
    }
    let mut i_right = i0+2;
    distance = 2;

    while hall.get(i_right) == Some(&0){
        v.push((i_right,distance));
        i_right+=1;
        distance+=if (2..6).contains(&(i_right as usize)){2}else{1};
    }
    v
}
fn distance_room(i0 : usize, room: u8, hall: &[u8])->Option<u32>{
    let room = room as f32 + 0.5;
    let mut i0 = i0  as f32;
    let mut dist = 0;
    let sign = (i0 - room).signum();
    if sign.is_sign_positive(){
    while i0-room>0.0{    
        i0-=sign;
        if hall[i0 as usize]!=0{
            return None
        }
        dist+=if (1..5).contains(&(i0 as usize)){2}else{1};
      
    }} else{
        while room-i0>0.0{      
            i0-=sign;
            if hall[i0 as usize]!=0{
                return None
            }
            dist+=if (2..6).contains(&(i0 as usize)){2}else{1};
           
        }
    }
    
    Some(dist)
}

#[cfg(test)]
mod day23 {

    use super::*;

    const INPUT: &'static str = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), 12521);
    }
    #[test]
    #[ignore]
    fn part_2() {
        assert_eq!(part2(INPUT), "");
    }
}
