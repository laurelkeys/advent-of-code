//! --- Day 16: Packet Decoder ---

use crate::solver::Solver;
use std::{
    io::{self, BufRead, BufReader},
    str,
};

/// https://adventofcode.com/2021/day/16
pub struct Day16;

fn u64_from_utf8_radix_2(utf8_radix2: &[u8]) -> u64 {
    let str_radix2 =
        str::from_utf8(utf8_radix2).unwrap_or_else(|_| panic!("utf8_radix2={:?}", &utf8_radix2));
    u64::from_str_radix(str_radix2, 2).unwrap_or_else(|_| panic!("str_radix2={:?}", &str_radix2))
}

#[derive(Debug)]
struct Packet {
    version: u64,
    kind: PacketKind,
}

#[derive(Debug)]
enum PacketKind {
    LiteralValue(u64),
    Operator(u64, Vec<Packet>),
}

fn parse_packet(packet: &[u8]) -> (Packet, usize) {
    let version = u64_from_utf8_radix_2(&packet[0..3]);
    let type_id = u64_from_utf8_radix_2(&packet[3..6]);

    let mut packet_length = 6;

    let kind = if type_id == 4 {
        let mut value = vec![];
        for chunk in packet[6..].chunks_exact(5) {
            packet_length += 5;
            value.extend_from_slice(&chunk[1..]);
            if chunk[0] == b'0' {
                break;
            }
        }
        PacketKind::LiteralValue(u64_from_utf8_radix_2(&value))
    } else {
        let length_type_id = packet[6];
        packet_length += 1;

        match length_type_id {
            b'0' => {
                // Total length in bits of the sub-packets contained by this packet.
                let subpackets_length = u64_from_utf8_radix_2(&packet[7..7 + 15]);
                packet_length += 15;

                let mut subpackets = vec![];
                while packet_length < subpackets_length as usize + 7 + 15 {
                    let (subpacket, subpacket_length) = parse_packet(&packet[packet_length..]);
                    packet_length += subpacket_length;
                    subpackets.push(subpacket);
                }
                PacketKind::Operator(type_id, subpackets)
            }
            b'1' => {
                // Number of sub-packets immediately contained by this packet.
                let number_of_subpackets = u64_from_utf8_radix_2(&packet[7..7 + 11]);
                packet_length += 11;

                let mut subpackets = vec![];
                while subpackets.len() < number_of_subpackets as usize {
                    let (subpacket, subpacket_length) = parse_packet(&packet[packet_length..]);
                    packet_length += subpacket_length;
                    subpackets.push(subpacket);
                }
                PacketKind::Operator(type_id, subpackets)
            }
            _ => unreachable!(),
        }
    };

    (Packet { version, kind }, packet_length)
}

impl Solver for Day16 {
    type Input = Vec<u8>;
    type Output1 = u64;
    type Output2 = u64;

    /// Decode the structure of your hexadecimal-encoded BITS transmission;
    /// what do you get if you add up the version numbers in all packets?
    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        fn sum_versions(packet: &Packet) -> u64 {
            match packet.kind {
                PacketKind::LiteralValue(_) => packet.version,
                PacketKind::Operator(_, ref subpackets) => {
                    subpackets.iter().fold(packet.version, |acc, subpacket| {
                        acc + sum_versions(subpacket)
                    })
                }
            }
        }

        sum_versions(&parse_packet(input).0)
    }

    /// What do you get if you evaluate the expression represented by your
    /// hexadecimal-encoded BITS transmission?
    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        fn eval(packet: &Packet) -> u64 {
            match packet.kind {
                PacketKind::LiteralValue(value) => value,
                PacketKind::Operator(type_id, ref subpackets) => match type_id {
                    0 => subpackets.iter().fold(0, |acc, p| acc + eval(p)),
                    1 => subpackets.iter().fold(1, |acc, p| acc * eval(p)),
                    2 => subpackets.iter().fold(u64::MAX, |acc, p| acc.min(eval(p))),
                    3 => subpackets.iter().fold(u64::MIN, |acc, p| acc.max(eval(p))),
                    5 => (eval(&subpackets[0]) > eval(&subpackets[1])) as u64,
                    6 => (eval(&subpackets[0]) < eval(&subpackets[1])) as u64,
                    7 => (eval(&subpackets[0]) == eval(&subpackets[1])) as u64,
                    _ => unreachable!(),
                },
            }
        }

        eval(&parse_packet(input).0)
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let input = BufReader::new(r).lines().flatten().next().unwrap();

        input
            .chars()
            .flat_map(|hex| match hex {
                '0' => [b'0', b'0', b'0', b'0'],
                '1' => [b'0', b'0', b'0', b'1'],
                '2' => [b'0', b'0', b'1', b'0'],
                '3' => [b'0', b'0', b'1', b'1'],
                '4' => [b'0', b'1', b'0', b'0'],
                '5' => [b'0', b'1', b'0', b'1'],
                '6' => [b'0', b'1', b'1', b'0'],
                '7' => [b'0', b'1', b'1', b'1'],
                '8' => [b'1', b'0', b'0', b'0'],
                '9' => [b'1', b'0', b'0', b'1'],
                'A' => [b'1', b'0', b'1', b'0'],
                'B' => [b'1', b'0', b'1', b'1'],
                'C' => [b'1', b'1', b'0', b'0'],
                'D' => [b'1', b'1', b'0', b'1'],
                'E' => [b'1', b'1', b'1', b'0'],
                'F' => [b'1', b'1', b'1', b'1'],
                _ => unreachable!(),
            })
            .collect::<Vec<u8>>()
    }
}
