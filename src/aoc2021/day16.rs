pub fn part1(input: &str) -> u32 {
    let mut n = parse(input);
    parse_packet(&mut n).ver
}

pub fn part2(input: &str) -> u64 {
    let mut n = parse(input);
    parse_packet(&mut n).value
}

fn parse_packet(n: &mut impl Iterator<Item = u8>) -> Packet {
    let mut ver = to_num(n.take(3));
    let id = to_num(n.take(3));
    let mut bits = 0;

    if id == 4 {
        bits += 11;
        let mut value = 0;
        while n.next().unwrap() == 1 {
            value = value << 4 | to_num(n.take(4)) as u64;
            bits += 5;
        }
        value = value << 4 | to_num(n.take(4)) as u64;
        return Packet { ver, value, bits };
    }
    let length_id = n.next().unwrap();
    let length = to_num(n.take(if length_id == 1 { 11 } else { 15 })) as u16;
    let children = match length_id {
        0 => {
            let mut children = vec![];
            while bits < length {
                let new_packet = parse_packet(n);
                bits += new_packet.bits;
                children.push(new_packet);
            }
            bits += 22;
            children
        }
        _ => {
            bits += 18;
            (0..length)
                .map(|_| {
                    let new_packet = parse_packet(n);
                    bits += new_packet.bits;
                    new_packet
                })
                .collect()
        }
    };
    let mut values = children.iter().map(|p| p.value);
    let value = match id {
        0 => values.sum::<u64>(),
        1 => values.product::<u64>(),
        2 => values.min().unwrap(),
        3 => values.max().unwrap(),
        5 => u64::from(values.next().unwrap() > values.next().unwrap()),
        6 => u64::from(values.next().unwrap() < values.next().unwrap()),
        7 => u64::from(values.next().unwrap() == values.next().unwrap()),
        e => panic!("unknown operation {e}"),
    };
    ver += children.into_iter().map(|p| p.ver).sum::<u32>();
    Packet { ver, value, bits }
}

fn to_num(bits: impl Iterator<Item = u8>) -> u32 {
    bits.fold(0, |acc, b| (b as u32) | acc << 1)
}

#[derive(Debug)]
struct Packet {
    ver: u32,
    value: u64,
    bits: u16,
}

fn parse(input: &str) -> impl Iterator<Item = u8> + '_ {
    let to_bin = |n| (0..4).rev().map(move |i| ((n >> i) & 1u8));
    input
        .chars()
        .flat_map(move |c| to_bin(c.to_digit(16).unwrap() as u8))
}

#[cfg(test)]
mod day16 {

    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(part1("A0016C880162017C3686B18A3D4780"), 31);
    }
    #[test]
    fn part_2() {
        assert_eq!(part2("9C0141080250320F1802104A08"), 1);
    }
}
