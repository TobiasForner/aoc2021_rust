use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct BitBuffer {
    hex_values: Vec<u64>,
    pos: usize,
    pos_in_hex: usize,
}

impl BitBuffer {
    fn new(s: &str) -> Self {
        const HEX: u32 = 16;
        let hex_values: Vec<u64> = s.chars().map(|x| x.to_digit(HEX).unwrap() as u64).collect();
        let pos = 0;
        let pos_in_hex = 0;
        BitBuffer {
            hex_values,
            pos,
            pos_in_hex,
        }
    }
    fn get_next_n(&mut self, n: u32) -> u64 {
        if self.pos >= self.hex_values.len() {
            panic!("Reached end of string!");
        }
        let mut res = 0;
        for _ in 0..n {
            res = res << 1;

            let current_hex = self.hex_values[self.pos];

            res += match self.pos_in_hex {
                0 => current_hex >> 3,
                1 => (current_hex & 4) >> 2,
                2 => (current_hex & 2) >> 1,
                3 => current_hex & 1,
                _ => panic!("Invalid hex pos!"),
            };
            self.pos_in_hex += 1;
            if self.pos_in_hex >= 4 {
                self.pos += 1;
                self.pos_in_hex = 0;
            }
        }
        res
    }
}

struct Packet {
    version: u64,
    content: PacketType,
    length: u64,
}

impl Packet {
    fn version_sum(&self) -> u64 {
        let res: u64 = match &self.content {
            PacketType::Literal(_) => self.version,
            PacketType::Op(_, packets) => {
                let sub_sums: u64 = packets.iter().map(|x| x.version_sum()).sum();
                self.version + sub_sums
            }
        };
        res
    }

    fn evaluate(&self) -> u64 {
        use PacketType::*;
        match &self.content {
            Literal(x) => *x,
            Op(op, packets) => match op {
                Operator::Sum => packets.iter().map(|x| x.evaluate()).sum(),
                Operator::Product => packets.iter().map(|x| x.evaluate()).product(),
                Operator::Minimum => packets.iter().map(|x| x.evaluate()).min().unwrap(),
                Operator::Maximum => packets.iter().map(|x| x.evaluate()).max().unwrap(),
                Operator::GT => {
                    if packets[0].evaluate() > packets[1].evaluate() {
                        1
                    } else {
                        0
                    }
                }
                Operator::LT => {
                    if packets[0].evaluate() < packets[1].evaluate() {
                        1
                    } else {
                        0
                    }
                }
                Operator::EQ => {
                    if packets[0].evaluate() == packets[1].evaluate() {
                        1
                    } else {
                        0
                    }
                }
            },
        }
    }
}
enum PacketType {
    Literal(u64),
    Op(Operator, Vec<Packet>),
}

enum Operator {
    Sum,
    Product,
    Minimum,
    Maximum,
    GT,
    LT,
    EQ,
}

fn parse_input(path: &str) -> Result<Packet> {
    let io = File::open(path)?;
    let br = BufReader::new(io);
    let mut lines = br.lines();
    let line = lines.next().unwrap()?;
    let mut buffer = BitBuffer::new(&line);
    Ok(parse_packet(&mut buffer))
}

fn parse_packet(buffer: &mut BitBuffer) -> Packet {
    let version = buffer.get_next_n(3);
    let typ = buffer.get_next_n(3);
    let mut length = 6;
    match typ {
        4 => {
            //literal
            //println!("Parsing literal (version {})", version);
            let mut content = 0;
            loop {
                let last = buffer.get_next_n(1) == 0;
                content = (content << 4) + buffer.get_next_n(4);
                length += 5;
                if last {
                    break;
                };
            }
            Packet {
                version: version,
                content: PacketType::Literal(content),
                length: length,
            }
        }
        _ => {
            //operator
            //println!("Parsing operator (version {})", version);
            let length_type = buffer.get_next_n(1);
            length += 1;
            let mut sub_packets: Vec<Packet> = vec![];
            if length_type == 0 {
                let nr_sub_bits = buffer.get_next_n(15);
                length += 15;
                let mut current_sub_len = 0;
                while current_sub_len < nr_sub_bits {
                    let p = parse_packet(buffer);
                    current_sub_len += p.length;
                    sub_packets.push(p);
                }
                length += current_sub_len;
            } else {
                let nr_packets = buffer.get_next_n(11);
                length += 11;
                for _ in 0..nr_packets {
                    let p = parse_packet(buffer);
                    length += p.length;
                    sub_packets.push(p);
                }
            }
            let op = match typ {
                0 => Operator::Sum,
                1 => Operator::Product,
                2 => Operator::Minimum,
                3 => Operator::Maximum,
                5 => Operator::GT,
                6 => Operator::LT,
                7 => Operator::EQ,
                _ => panic!("invalid operator type!"),
            };
            Packet {
                version: version,
                content: PacketType::Op(op, sub_packets),
                length: length,
            }
        }
    }
}

pub fn part1(path: &str) -> Result<u64> {
    let packet = parse_input(path)?;
    Ok(packet.version_sum())
}

pub fn part2(path: &str) -> Result<u64> {
    let packet = parse_input(path)?;
    Ok(packet.evaluate())
}
