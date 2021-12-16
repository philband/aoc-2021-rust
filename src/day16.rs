use std::collections::{HashMap, VecDeque};
use itertools::Itertools;
use crate::day16::State::*;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Point(i32, i32);
type Data = Vec<char>;


#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Packet {
    v: u64,
    t: u64,
    data: u64,
    nsub: u32,
    sub: Vec<Packet>,
}

impl Packet {
    pub fn new() -> Self {
        Packet{v: 0, t: 0, data: 0, nsub: 0, sub: Vec::new()}
    }

    pub fn version_sum(&self) -> u64 {
        self.v + self.sub.iter().map(|s| s.version_sum()).sum::<u64>() as u64
    }

    pub fn eval(&self) -> u64 {
        match self.t {
            0 => self.sub.iter().map(|p| p.eval()).sum(),
            1 => self.sub.iter().map(|p| p.eval()).product(),
            2 => self.sub.iter().map(|p| p.eval()).min().unwrap(),
            3 => self.sub.iter().map(|p| p.eval()).max().unwrap(),
            5 | 6 | 7 => {
                let (a, b) = (&self.sub[0], &self.sub[1]);
                match self.t {
                    5 if a.eval() > b.eval() => 1,
                    6 if a.eval() < b.eval() => 1,
                    7 if a.eval() == b.eval() => 1,
                    _ => 0
                }
            },
            4 => self.data,
            _ => unreachable!()
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum State {
    Init,
    ParseVersion,
    ParseType,
    ParseLiteralData,
    ParseOperator,
    ParseSubLen,
    ParseSubN,
    Finished,
}

#[aoc_generator(day16)]
pub fn generator(input: &str) -> Data {
    let binary = hex::decode(input).unwrap();
    binary.into_iter().flat_map(|x| {
        (0..8).rev().into_iter().map(|n|{
            if (1 << n) & x > 0 {
                '1'
            } else {
                '0'
            }
        }).collect::<Vec<char>>()
    }).collect::<Vec<char>>()

}

pub fn to_num(bin: &[char]) -> u64 {
    let mut num = 0;
    for (n, c) in bin.iter().rev().enumerate() {
        if *c == '1' {
            num += 1 << n;
        }
    }
    num
}


pub fn state_machine(inputs: &Data, mut pos: &mut usize, initial: State) -> Vec<Packet> {
    let mut state = initial;
    let mut p = Packet::new();

    let mut packets = Vec::<Packet>::new();

    while state != State::Finished  {
        match state {
            Init => {
                state = ParseVersion;
            },
            ParseVersion => {
                p.v = to_num(&inputs[*pos..*pos+3]);
                *pos += 3;
                state = ParseType;
            },
            ParseType => {
                p.t = to_num(&inputs[*pos..*pos+3]);
                *pos += 3;
                state = match p.t {
                    4 => ParseLiteralData,
                    _ => ParseOperator
                };
            },
            ParseOperator => {
                state = match inputs[*pos] {
                    '1' => ParseSubN,
                    '0' => ParseSubLen,
                    _ => unreachable!()
                };
                *pos += 1;
            },
            ParseLiteralData => {
                let mut res = Vec::<char>::new();
                let mut last = false;
                while ! last {
                    if inputs[*pos] == '0' {
                        last = true;
                    }
                    res.append(&mut inputs[*pos+1..*pos+5].to_vec().clone());
                    *pos += 5;
                }
                p.data = to_num(res.as_slice());
                state = Finished;
            },
            ParseSubN => {
                let n = to_num(&inputs[*pos..*pos+11]);
                *pos += 11;
                for i in 1..=n {
                    println!("Recusing for {}/{} n packets at pos {}", i, n, *pos);
                    let ps = state_machine(inputs, &mut pos, Init);
                    assert_eq!(ps.len(), 1);
                    p.sub.push(ps[0].clone());
                }
                state = Finished;
            }
            ParseSubLen => {
                let len = to_num(&inputs[*pos..*pos+15]);
                *pos += 15;
                let target = *pos + len as usize;
                for i in 1.. {
                    println!("Recusing for {} -- {} len packets at pos {}", i, len, *pos);
                    let ps = state_machine(inputs, &mut pos, Init);
                    assert_eq!(ps.len(), 1);
                    p.sub.push(ps[0].clone());
                    if *pos == target {
                        break;
                    }
                }
                state = Finished;
            }
            State::Finished => {
            }
            _ => unreachable!(),
        }
    }
    packets.push(p.clone());
    packets
}



#[aoc(day16, part1)]
pub fn part1(inputs: &Data) -> u64 {
    let mut pos = 0;

    let packets = state_machine(inputs, &mut pos, Init);

    for p in &packets {
        print!("Packet V {} T {} Data {}", p.v, p.t, p.data);
        print!("{:?}", p.sub)
    }

    packets[0].version_sum()
    //69
}

#[aoc(day16, part2)]
pub fn part2(inputs: &Data) -> u64 {

    let mut pos = 0;

    let packets = state_machine(inputs, &mut pos, Init);

    for p in &packets {
        print!("Packet V {} T {} Data {}", p.v, p.t, p.data);
        print!("{:?}", p.sub)
    }

    packets[0].eval()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test1() {
        let input = generator("D2FE28");
        assert_eq!(part1(&input), 6);
    }

    #[test]
    pub fn test1_2() {
        let input = generator("38006F45291200");
        assert_eq!(part1(&input), 9);
    }

    #[test]
    pub fn test1_3() {
        let input = generator("EE00D40C823060");
        assert_eq!(part1(&input), 14);
    }

    #[test]
    pub fn test1_4() {
        let input = generator("8A004A801A8002F478");
        assert_eq!(part1(&input), 16);
    }

    #[test]
    pub fn test1_5() {
        let input = generator("620080001611562C8802118E34");
        assert_eq!(part1(&input), 12);
    }

    #[test]
    pub fn test1_6() {
        let input = generator("C0015000016115A2E0802F182340");
        assert_eq!(part1(&input), 23);
    }

    #[test]
    pub fn test1_7() {
        let input = generator("A0016C880162017C3686B18A3D4780");
        assert_eq!(part1(&input), 31);
    }

    #[test]
    pub fn test2_1() {
        let input = generator("C200B40A82");
        assert_eq!(part2(&input), 3);
    }

    #[test]
    pub fn test2_2() {
        let input = generator("04005AC33890");
        assert_eq!(part2(&input), 54);
    }

    #[test]
    pub fn test2_3() {
        let input = generator("880086C3E88112");
        assert_eq!(part2(&input), 7);
    }

    #[test]
    pub fn test2_4() {
        let input = generator("CE00C43D881120");
        assert_eq!(part2(&input), 9);
    }

    #[test]
    pub fn test2_5() {
        let input = generator("D8005AC2A8F0");
        assert_eq!(part2(&input), 1);
    }

    #[test]
    pub fn test2_6() {
        let input = generator("F600BC2D8F");
        assert_eq!(part2(&input), 0);
    }

    #[test]
    pub fn test2_7() {
        let input = generator("9C005AC2F8F0");
        assert_eq!(part2(&input), 0);
    }

    #[test]
    pub fn test2_8() {
        let input = generator("9C0141080250320F1802104A08");
        assert_eq!(part2(&input), 1);
    }

}
