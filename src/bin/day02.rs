use adventofcode::{self as aoc, aoc_problem, Solution};
use std::str::{FromStr, Lines};

enum Instruction {
    Fwd(i32),
    Down(i32),
    Up(i32),
}

impl FromStr for Instruction {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cmd, val) = s.split_once(' ').expect("exactly 1 delimiter expected");
        let val = val.parse::<i32>()?;
        let instr = match cmd {
            "forward" => Instruction::Fwd(val),
            "down" => Instruction::Down(val),
            "up" => Instruction::Up(val),
            _ => unreachable!(),
        };
        Ok(instr)
    }
}

struct Solver;

impl Solution for Solver {
    type Output = i32;

    fn solve_a(&self, input: Lines) -> i32 {
        let mut pos = 0;
        let mut depth = 0;
        for line in input {
            let cmd: Instruction = line.parse().unwrap();
            match cmd {
                Instruction::Fwd(v) => {
                    pos += v;
                }
                Instruction::Down(v) => {
                    depth += v;
                }
                Instruction::Up(v) => {
                    depth -= v;
                }
            }
        }
        pos * depth
    }

    fn solve_b(&self, input: Lines) -> i32 {
        let mut pos = 0;
        let mut depth = 0;
        let mut aim = 0;
        for line in input {
            let cmd: Instruction = line.parse().unwrap();
            match cmd {
                Instruction::Fwd(v) => {
                    pos += v;
                    depth += v * aim;
                }
                Instruction::Down(v) => {
                    aim += v;
                }
                Instruction::Up(v) => {
                    aim -= v;
                }
            }
        }
        pos * depth
    }
}

// Don't forget to set AOC_TOKEN
fn main() {
    let problem = aoc_problem!(year = 2021, day = 2);
    let solver = Solver {};

    match aoc::earn_star(problem.part_b(), solver) {
        Ok(resp) => println!("Response: {}", resp),
        Err(err) => panic!("Something went wrong: {}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::StripMargin;

    #[test]
    fn test_solution() {
        let solver = Solver {};
        let input = r"
            |forward 5
            |down 5
            |forward 8
            |up 3
            |down 8
            |forward 2
        "
        .trim()
        .strip_margin();

        let ans = solver.solve_a(input.lines());
        assert_eq!(ans, 150);

        let ans = solver.solve_b(input.lines());
        assert_eq!(ans, 900);
    }
}
