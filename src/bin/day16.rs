use adventofcode::{self as aoc, aoc_problem, make_err, Solution};
use itertools::Itertools;
use std::str::Lines;

#[derive(Debug)]
enum Op {
    Sum,
    Product,
    Min,
    Max,
    Greater,
    Less,
    Equal,
}

impl TryFrom<u8> for Op {
    type Error = aoc::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let op = match value {
            0 => Self::Sum,
            1 => Self::Product,
            2 => Self::Min,
            3 => Self::Max,
            5 => Self::Greater,
            6 => Self::Less,
            7 => Self::Equal,
            _ => {
                return make_err!("Unexpected id: {}", value);
            }
        };
        Ok(op)
    }
}

#[derive(Debug)]
struct Packet {
    version: u64,
    bits: usize,
    value: Value,
}

#[derive(Debug)]
enum Value {
    Literal(u64),
    Expression(Op, Vec<Packet>),
}

impl Packet {
    fn parse_literal_parts(input: &[char]) -> (u64, usize) {
        let input = &input[6..];
        let last = input
            .iter()
            .enumerate()
            .find(|&(i, &el)| i % 5 == 0 && el == '0')
            .unwrap()
            .0
            + 5;

        let v = input
            .iter()
            .enumerate()
            .filter(|&(i, _)| i % 5 != 0 && i < last)
            .map(|(_, &c)| c)
            .collect::<String>();
        let v = u64::from_str_radix(&v, 2).unwrap();

        (v, last + 6)
    }

    fn parse_operator_parts(input: &[char]) -> (Vec<Packet>, usize) {
        let input = &input[6..];
        let mut elements = vec![];
        let mut read = 0;
        match input[0] {
            '0' => {
                // length_id = 0
                let total_len = input[1..16].iter().collect::<String>();
                let total_len = usize::from_str_radix(&total_len, 2).unwrap();
                let inp = &input[16..16 + total_len];
                while read < total_len {
                    elements.push(Packet::from(&inp[read..]));
                    read += elements.last().unwrap().bits;
                }

                read += 22;
            }
            '1' => {
                // length_id = 1
                let total_num = input[1..12].iter().collect::<String>();
                let total_num = usize::from_str_radix(&total_num, 2).unwrap();
                let inp = &input[12..];
                for _ in 0..total_num {
                    elements.push(Packet::from(&inp[read..]));
                    read += elements.last().unwrap().bits;
                }

                read += 18;
            }
            _ => {
                unreachable!()
            }
        }
        (elements, read)
    }

    fn evaluate(&self) -> u64 {
        match &self.value {
            Value::Literal(num) => *num,
            Value::Expression(op, elements) => {
                let evaled = elements.iter().map(|el| el.evaluate());
                match op {
                    Op::Sum => evaled.sum(),
                    Op::Product => evaled.product(),
                    Op::Min => evaled.min().unwrap(),
                    Op::Max => evaled.max().unwrap(),
                    Op::Greater => evaled.tuple_windows().all(|(a, b)| a > b) as u64,
                    Op::Less => evaled.tuple_windows().all(|(a, b)| a < b) as u64,
                    Op::Equal => evaled.into_iter().all_equal() as u64,
                }
            }
        }
    }
}

impl From<&[char]> for Packet {
    fn from(input: &[char]) -> Self {
        let version = {
            let v = input[0..3].iter().collect::<String>();
            u64::from_str_radix(&v, 2).unwrap()
        };
        let packet_id = {
            let v = input[3..6].iter().collect::<String>();
            u8::from_str_radix(&v, 2).unwrap()
        };

        match packet_id {
            4 => {
                let (value, bits) = Packet::parse_literal_parts(input);
                Self {
                    version,
                    bits,
                    value: Value::Literal(value),
                }
            }
            pid => {
                let op = Op::try_from(pid).unwrap();
                let (elements, bits) = Packet::parse_operator_parts(input);
                Self {
                    version,
                    bits,
                    value: Value::Expression(op, elements),
                }
            }
        }
    }
}

fn add_versions(p: &Packet) -> u64 {
    match &p.value {
        Value::Literal(_) => p.version,
        Value::Expression(_, elts) => {
            p.version + elts.iter().map(add_versions).sum::<u64>()
        }
    }
}

struct Solver;

impl Solver {
    fn parse_input(&self, input: Lines) -> Vec<char> {
        let msg = input.last().unwrap();
        let msg = msg
            .chars()
            .map(|c| c.to_digit(16).unwrap())
            .map(|n| format!("{:04b}", n))
            .collect::<Vec<_>>()
            .join("");

        msg.chars().collect()
    }
}

impl Solution for Solver {
    type Output = u64;

    fn solve_a(&self, input: Lines) -> u64 {
        let msg = self.parse_input(input);
        let packet = Packet::from(&*msg);

        add_versions(&packet)
    }

    fn solve_b(&self, input: Lines) -> u64 {
        let msg = self.parse_input(input);
        let packet = Packet::from(&*msg);

        packet.evaluate()
    }
}

// Don't forget to set AOC_TOKEN
fn main() {
    let problem = aoc_problem!(year = 2021, day = 16);
    let solver = Solver {};

    match aoc::earn_star(problem.part_b(), solver) {
        Ok(resp) => println!("Response: {}", resp),
        Err(err) => panic!("Something went wrong: {}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_a() {
        let solver = Solver {};
        let test_data = [
            ("D2FE28", 6),
            ("8A004A801A8002F478", 16),
            ("620080001611562C8802118E34", 12),
            ("C0015000016115A2E0802F182340", 23),
            ("A0016C880162017C3686B18A3D4780", 31),
        ];

        test_data.iter().for_each(|&(input, ans)| {
            assert_eq!(solver.solve_a(input.lines()), ans);
        });
    }

    #[test]
    fn test_solution_b() {
        let solver = Solver {};
        let test_data = [
            ("C200B40A82", 3),
            ("04005AC33890", 54),
            ("880086C3E88112", 7),
            ("CE00C43D881120", 9),
            ("D8005AC2A8F0", 1),
            ("F600BC2D8F", 0),
            ("9C005AC2F8F0", 0),
            ("9C0141080250320F1802104A08", 1),
        ];

        test_data.iter().for_each(|&(input, ans)| {
            assert_eq!(solver.solve_b(input.lines()), ans);
        });
    }
}
