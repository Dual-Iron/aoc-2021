use PacketData::*;
enum PacketData {
    Literal(u64),
    Operate(Vec<Packet>),
}

struct Packet {
    v: u8, // version
    t: u8, // type id
    d: PacketData
}

fn parse_input() -> Vec<u8> {
    let mut ret = Vec::new();
    for c in crate::input!(16).chars() {
        let n = c.to_digit(16).unwrap() as u8;
        ret.extend([(n & 8) >> 3, (n & 4) >> 2, (n & 2) >> 1, n & 1]);
    }
    ret
}

fn get_num(bytes: &[u8]) -> u64 {
    bytes
        .iter()
        .enumerate()
        .map(|(i, &n)| (n as u64) << (bytes.len() - 1 - i))
        .sum()
}

fn parse_packet(b: &[u8], i: &mut usize) -> Packet {
    let my_bytes = &b[*i..];
    let v = get_num(&my_bytes[0..3]) as u8;
    let t = get_num(&my_bytes[3..6]) as u8;

    if t == 4 {
        let mut n_i = 6;
        let mut n = 0;
        loop {
            let stop = my_bytes[n_i] == 0;
            n <<= 4;
            n |= get_num(&my_bytes[n_i + 1..=n_i + 4]);
            n_i += 5;
            if stop {
                *i += n_i;
                break Packet { v, t, d: Literal(n) };
            }
        }
    } else if my_bytes[6] == 1 { 
        *i += 18;
        let n = get_num(&my_bytes[7..18]);
        let mut sub = Vec::with_capacity(n as usize);
        for _ in 0..n {
            sub.push(parse_packet(b, i));
        }
        Packet { v, t, d: Operate(sub) }
    } else {
        *i += 22;
        let i_max = *i + get_num(&my_bytes[7..22]) as usize;
        let mut sub = Vec::new();
        while *i < i_max {
            sub.push(parse_packet(b, i));
        }
        Packet { v, t, d: Operate(sub) }
    }
}

// 854
pub fn part1() {
    fn version_sum(packet: Packet) -> u32 {
        match packet.d {
            Literal(_) => packet.v as u32,
            Operate(d) => d.into_iter().map(version_sum).sum()
        }
    }

    let packet = parse_packet(&parse_input(), &mut 0);

    println!("{}", version_sum(packet));
}

fn eval(packet: Packet) -> u64 {
    match packet.d {
        Literal(n) => n,
        Operate(d) => {
            let mut sub = d.into_iter().map(eval);

            match packet.t {
                0 => sub.sum(),
                1 => sub.product(),
                2 => sub.min().unwrap(),
                3 => sub.max().unwrap(),
                5 => (sub.next() > sub.next()).into(),
                6 => (sub.next() < sub.next()).into(),
                7 => (sub.next() == sub.next()).into(),
                _ => panic!("bad packet type {}", packet.t),
            }
        }
    }
}

// 186189840660
pub fn part2() {
    let packet = parse_packet(&parse_input(), &mut 0);

    println!("{}", eval(packet));
}
