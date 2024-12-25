use std::{
    collections::VecDeque,
    io::{Cursor, Seek},
};

use bytes::Buf;

use crate::SolveInfo;

pub fn run(input: &str) -> anyhow::Result<SolveInfo> {
    Ok(SolveInfo {
        part01: part01(input).to_string(),
        part02: part02(input).to_string(),
    })
}

pub fn part01(input: &str) -> i64 {
    let bin = to_binary(input);
    let packet = parse(&mut Cursor::new(&bin[..])).unwrap();

    let mut version_sum = 0i64;

    let mut q = VecDeque::new();
    q.push_front(packet);

    while let Some(packet) = q.pop_front() {
        version_sum += match packet.packet_type {
            PacketType::Literal(_) => packet.version as i64,
            PacketType::Sum(subpackets)
            | PacketType::Product(subpackets)
            | PacketType::Minimum(subpackets)
            | PacketType::Maximum(subpackets)
            | PacketType::LessThan(subpackets)
            | PacketType::GreaterThan(subpackets)
            | PacketType::EqualTo(subpackets) => {
                for p in subpackets {
                    q.push_back(p);
                }
                packet.version as i64
            }
        };
    }

    version_sum
}

pub fn part02(input: &str) -> i64 {
    let bin = to_binary(input);
    let packet = parse(&mut Cursor::new(&bin[..])).unwrap();
    packet.compute() as i64
}

#[derive(Debug, PartialEq)]
struct Packet {
    version: u8,
    packet_type: PacketType,
}

#[derive(Debug, PartialEq)]
enum PacketType {
    Literal(u64),
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Minimum(Vec<Packet>),
    Maximum(Vec<Packet>),
    GreaterThan(Vec<Packet>),
    LessThan(Vec<Packet>),
    EqualTo(Vec<Packet>),
}

impl Packet {
    fn compute(&self) -> u64 {
        match &self.packet_type {
            PacketType::Literal(n) => *n,
            PacketType::Sum(packets) => packets.iter().map(|p| p.compute()).sum(),
            PacketType::Product(packets) => packets.iter().map(|p| p.compute()).product(),
            PacketType::Minimum(packets) => packets.iter().map(|p| p.compute()).min().unwrap(),
            PacketType::Maximum(packets) => packets.iter().map(|p| p.compute()).max().unwrap(),
            PacketType::GreaterThan(packets) => {
                assert_eq!(2, packets.len());
                u64::from(packets[0].compute() > packets[1].compute())
            }
            PacketType::LessThan(packets) => {
                assert_eq!(2, packets.len());
                u64::from(packets[0].compute() < packets[1].compute())
            }
            PacketType::EqualTo(packets) => {
                assert_eq!(2, packets.len());
                u64::from(packets[0].compute() == packets[1].compute())
            }
        }
    }
}

fn parse(src: &mut Cursor<&[u8]>) -> anyhow::Result<Packet> {
    let version = get_u8(src, 3);
    let ptype = get_u8(src, 3);
    match ptype {
        4 => parse_literal(src, version),
        _ => parse_operator(src, version, ptype),
    }
}

fn parse_literal(src: &mut Cursor<&[u8]>, version: u8) -> anyhow::Result<Packet> {
    let mut n: u64 = 0;
    loop {
        let cont = src.get_u8();
        n <<= 4;
        n |= get_u8(src, 4) as u64;
        if cont == 0 {
            break;
        }
    }

    Ok(Packet {
        version,
        packet_type: PacketType::Literal(n),
    })
}

fn parse_operator(src: &mut Cursor<&[u8]>, version: u8, type_id: u8) -> anyhow::Result<Packet> {
    let length_type_id = src.get_u8();
    let subpackets = match length_type_id {
        0 => parse_subpackets_bits(src)?,
        1 => parse_operator_total(src)?,
        _ => unimplemented!(),
    };

    let packet_type = match type_id {
        0 => PacketType::Sum(subpackets),
        1 => PacketType::Product(subpackets),
        2 => PacketType::Minimum(subpackets),
        3 => PacketType::Maximum(subpackets),
        5 => PacketType::GreaterThan(subpackets),
        6 => PacketType::LessThan(subpackets),
        7 => PacketType::EqualTo(subpackets),
        _ => unimplemented!(),
    };

    Ok(Packet {
        version,
        packet_type,
    })
}

// parse operator subpackets by the total number of bits in them
fn parse_subpackets_bits(src: &mut Cursor<&[u8]>) -> anyhow::Result<Vec<Packet>> {
    let mut subpackets = Vec::new();
    let subpacket_bits = get_u32(src, 15) as u64;
    let mut size_parsed = 0u64;
    while size_parsed < subpacket_bits {
        let packet_start_pos = src.stream_position().unwrap();
        let packet = parse(src)?;
        size_parsed += src.stream_position().unwrap() - packet_start_pos;
        subpackets.push(packet);
    }
    Ok(subpackets)
}

// parse operator subpackets by number that exist
fn parse_operator_total(src: &mut Cursor<&[u8]>) -> anyhow::Result<Vec<Packet>> {
    let mut subpackets = Vec::new();
    let nsubpackets = get_u32(src, 11);
    for _ in 0..nsubpackets {
        let packet = parse(src)?;
        subpackets.push(packet);
    }
    Ok(subpackets)
}

fn get_u8(src: &mut Cursor<&[u8]>, nbits: usize) -> u8 {
    get_u32(src, nbits) as u8
}

fn get_u32(src: &mut Cursor<&[u8]>, nbits: usize) -> u32 {
    let mut n = 0u32;
    for _ in 0..nbits {
        n <<= 1;
        n |= src.get_u8() as u32;
    }
    n
}

fn to_binary(hex: &str) -> Vec<u8> {
    let mut binary: Vec<u8> = Vec::with_capacity(hex.len() * 4);
    for ch in hex.chars() {
        let bits: [u8; 4] = match ch {
            '0' => [0, 0, 0, 0],
            '1' => [0, 0, 0, 1],
            '2' => [0, 0, 1, 0],
            '3' => [0, 0, 1, 1],
            '4' => [0, 1, 0, 0],
            '5' => [0, 1, 0, 1],
            '6' => [0, 1, 1, 0],
            '7' => [0, 1, 1, 1],
            '8' => [1, 0, 0, 0],
            '9' => [1, 0, 0, 1],
            'A' => [1, 0, 1, 0],
            'B' => [1, 0, 1, 1],
            'C' => [1, 1, 0, 0],
            'D' => [1, 1, 0, 1],
            'E' => [1, 1, 1, 0],
            'F' => [1, 1, 1, 1],
            '\n' => continue,
            _ => unreachable!(),
        };
        bits.iter().for_each(|b| binary.push(*b));
    }
    binary
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day16.input.txt");

    #[test]
    fn test_to_binary_string() {
        let binstr = to_binary("D2FE28");
        let expected: Vec<u8> = "110100101111111000101000"
            .chars()
            .map(|c| if c == '1' { 1 } else { 0 })
            .collect();
        assert_eq!(expected, binstr);
    }

    #[test]
    fn test_parse_literal() {
        let bin = to_binary("D2FE28");
        let expected = Packet {
            version: 6,
            packet_type: PacketType::Literal(2021),
        };
        let mut c = Cursor::new(&bin[..]);
        assert_eq!(expected, parse(&mut c).unwrap());
    }

    #[test]
    fn test_parse_operator_1() {
        let bin = to_binary("EE00D40C823060");
        let expected = Packet {
            version: 7,
            packet_type: PacketType::Maximum(vec![
                Packet {
                    version: 2,
                    packet_type: PacketType::Literal(1),
                },
                Packet {
                    version: 4,
                    packet_type: PacketType::Literal(2),
                },
                Packet {
                    version: 1,
                    packet_type: PacketType::Literal(3),
                },
            ]),
        };
        let mut c = Cursor::new(&bin[..]);
        assert_eq!(expected, parse(&mut c).unwrap());
    }

    #[test]
    fn test_part_one() {
        assert_eq!(16, part01("8A004A801A8002F478"));
        assert_eq!(12, part01("620080001611562C8802118E34"));
        assert_eq!(23, part01("C0015000016115A2E0802F182340"));
        assert_eq!(31, part01("A0016C880162017C3686B18A3D4780"));

        // puzzle input
        assert_eq!(897, part01(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(3, part02("C200B40A82"));
        assert_eq!(54, part02("04005AC33890"));
        assert_eq!(7, part02("880086C3E88112"));
        assert_eq!(9, part02("CE00C43D881120"));
        assert_eq!(1, part02("D8005AC2A8F0"));
        assert_eq!(0, part02("F600BC2D8F"));
        assert_eq!(0, part02("9C005AC2F8F0"));
        assert_eq!(1, part02("9C0141080250320F1802104A08"));

        // puzzle input
        assert_eq!(9485076995911, part02(INPUT));
    }
}
