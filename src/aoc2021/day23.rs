use std::{collections::{BinaryHeap}, cmp::Ordering};

pub fn part1(input: &str) -> u32 {
    let mut pos = [vec![], vec![],vec![],vec![],vec![0;7]];
    input.lines().rev().skip(1).take(2).enumerate().for_each(|(j,l)|l.trim().split('#').filter(|s|!s.is_empty()).enumerate().for_each(|(i,a)|pos[i].push(a.chars().next().unwrap() as u8 -b'A'+1)));
    return 0/*let mut heap = BinaryHeap::from([State{pos: pos.into_iter().flatten().collect::<Vec<_>>().try_into().unwrap(), energy: 0}]);
    while let Some(state) = heap.pop(){
        if state.pos.chunks(2).take(4).enumerate().all(|(i,room)|room.iter().all(|&ar|ar==i as u8+1)){
           //println!("{:?}",row);
            return state.energy;
        }
        for (i,a) in state.pos.iter().skip(8).enumerate().filter(|(_,a)|**a!=0){
            let index = 2*(*a as usize-1);
            if state.pos[index]==0 || (state.pos[index]==*a &&  state.pos[index+1]==0){
               if  let Some(distance) = distance_room(i, *a, &state.pos[8..]){
                let mut new_pos = state.pos.clone();
                new_pos[8+i] = 0;
                if new_pos[index]==0{
                    new_pos[index] = *a;
                    //let mut new_row = row.clone();
                    /*new_row.push(new_pos.clone());
                    new_row.push([1+distance as u8;15]);*/
                    heap.push(State{pos: new_pos, energy: state.energy+ (1+distance)*10u32.pow(*a as u32-1)});
                } else{
                    new_pos[index+1] = *a;
                    //let mut new_row = row.clone();
                    /*new_row.push(new_pos.clone());
                    new_row.push([distance as u8;15]);*/
                    heap.push(State{pos: new_pos, energy: state.energy+ distance*10u32.pow(*a as u32-1)});
                }          
            }
        }
        }
        for (i,room) in state.pos.chunks(2).take(4).enumerate(){
            if  room.iter().any(|&a|a!=0 && a!=i as u8+1){
                let possible_pos = distance_row(i, &state.pos[8..]);
                for (pos, distance) in possible_pos{              
                    let mut new_pos = state.pos.clone();
                    if new_pos[2*i+1]==0{
                        let a = new_pos[2*i];
                        new_pos[2*i] = 0;
                        new_pos[8+pos] = a;
                        /*let mut new_row = row.clone();
                    new_row.push(new_pos.clone());
                    new_row.push([1+distance as u8;15]);*/
                        heap.push(State{pos: new_pos, energy: state.energy+ (1+distance)*10u32.pow(a as u32-1)});
                    } else{
                        let a = new_pos[2*i+1] ;
                        new_pos[2*i+1] = 0;
                        new_pos[8+pos] = a;
                        /*let mut new_row = row.clone();
                        new_row.push(new_pos.clone());
                        new_row.push([distance as u8;15]);*/
                        heap.push(State{pos: new_pos, energy: state.energy+ distance*10u32.pow(a as u32-1)});
                    }              
                }
            }
        }

    }
    panic!("not found")*/
}

pub fn part2(input: &str) -> u32 {
    let mut pos = [vec![], vec![],vec![],vec![],vec![0;7]];
    let (i1,i2) = input.split_once("  ").unwrap();
    let insert ="  #D#C#B#A#\n  #D#B#A#C#\n  ";
    [i1,insert,i2].join("").lines().rev().skip(1).take(4).enumerate().for_each(|(j,l)|l.trim().split('#').filter(|s|!s.is_empty()).enumerate().for_each(|(i,a)|pos[i].push(a.chars().next().unwrap() as u8 -b'A'+1)));
    let l = pos[0].len();
    dbg!(l);
    let mut heap = BinaryHeap::from([State{pos: pos.into_iter().flatten().collect::<Vec<_>>().try_into().unwrap(), energy: 0}]);
    while let Some(state) = heap.pop(){
        if state.pos.chunks(l).take(4).enumerate().all(|(i,room)|room.iter().all(|&ar|ar==i as u8+1)){
           //println!("{:?}",row);
            return state.energy;
        }
        for (i,a) in state.pos.iter().skip(4*l).enumerate().filter(|(_,a)|**a!=0){
            let index = l*(*a as usize-1);
            if state.pos[index..index+1].iter().all(|ar|ar==a || ar==&0){
               if  let Some(distance) = distance_room(i, *a, &state.pos[4*l..]){
                let mut new_pos = state.pos.clone();
                new_pos[4*l+i] = 0;
                    let zero = new_pos[index..index+l].iter().position(|&ar|ar==0).unwrap_or(l-1);
                    new_pos[index+zero] = *a;
                    //let mut new_row = row.clone();
                    /*new_row.push(new_pos.clone());
                    new_row.push([1+distance as u8;15]);*/
                    heap.push(State{pos: new_pos, energy: state.energy+ ((l-zero-1) as u32+distance)*10u32.pow(*a as u32-1)});
                
            }
        }
        }
        for (i,room) in state.pos.chunks(l).take(4).enumerate(){
            if  room.iter().any(|&a|a!=0 && a!=i as u8+1){
                let possible_pos = distance_row(i, &state.pos[4*l..]);
                for (pos, distance) in possible_pos{              
                    let mut new_pos = state.pos.clone();
                    let last = new_pos[l*i..l*i+l].iter().rposition(|&ar|ar!=0).unwrap();
                        let a = new_pos[l*i+last];
                        new_pos[l*i+last] = 0;
                        new_pos[4*l+pos] = a;
                        /*let mut new_row = row.clone();
                    new_row.push(new_pos.clone());
                    new_row.push([1+distance as u8;15]);*/
                        heap.push(State{pos: new_pos, energy: state.energy+ ((l-1-last) as u32+distance)*10u32.pow(a as u32-1)});          
                }
            }
        }

    }
    panic!("not found")
}

#[derive(Eq, PartialEq)]
struct State {
    pos:[u8;4*4+7],
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
        if i0-room>0.0&&hall[i0 as usize]!=0{
            return None
        }
        dist+=if (1..5).contains(&(i0 as usize)){2}else{1};
      
    }} else{
        while room-i0>0.0{      
            i0-=sign;
            if  room-i0>0.0&&hall[i0 as usize]!=0{
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
    fn part_2() {
        assert_eq!(part2(INPUT), 44169);
    }
}
