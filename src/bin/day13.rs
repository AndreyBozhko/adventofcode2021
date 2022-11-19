use adventofcode::{self as aoc, aoc_problem, Solution};
use std::collections::HashSet;
use std::fmt;
use std::str::Lines;

struct Solver;

type Dots = HashSet<(u32, u32)>;
type Instructions = Vec<(String, u32)>;

#[derive(Debug, Eq, PartialEq)]
enum Answer {
    Num(usize),
    Str(String),
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Answer::Num(n) => {
                write!(f, "{}", n)
            }
            Answer::Str(s) => {
                panic!(
                    "\n\nCheck the answer below and submit manually:\n\n{}\n\n",
                    s
                )
            }
        }
    }
}

impl Solver {
    fn parse_input(&self, input: Lines) -> (Dots, Instructions) {
        let dots: Dots = input
            .clone()
            .filter(|line| line.contains(','))
            .map(|line| line.split_once(',').expect("Expect one delimiter"))
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
            .collect();

        let instr: Instructions = input
            .filter(|line| line.contains('='))
            .map(|line| {
                line.strip_prefix("fold along ")
                    .expect("Expect valid prefix")
                    .split_once('=')
                    .expect("Expect one delimiter")
            })
            .map(|(axis, pos)| (axis.to_string(), pos.parse().unwrap()))
            .collect();

        (dots, instr)
    }
}

impl Solution for Solver {
    type Output = Answer;

    fn solve_a(&self, input: Lines) -> Answer {
        let (dots, instr) = self.parse_input(input);
        let (axis, pos) = &instr[0];

        let folded: HashSet<_> = dots
            .iter()
            .map(|&(x, y)| match axis.as_str() {
                "x" if x > *pos => (2 * pos - x, y),
                "y" if y > *pos => (x, 2 * pos - y),
                _ => (x, y),
            })
            .collect();

        Answer::Num(folded.len())
    }

    fn solve_b(&self, input: Lines) -> Answer {
        let (dots, instr) = self.parse_input(input);

        let folded: HashSet<_> = dots
            .iter()
            .map(|&(xx, yy)| {
                instr
                    .iter()
                    .fold((xx, yy), |(x, y), (axis, pos)| match axis.as_str() {
                        "x" if x > *pos => (2 * pos - x, y),
                        "y" if y > *pos => (x, 2 * pos - y),
                        _ => (x, y),
                    })
            })
            .collect();

        let xmax = instr.iter().rev().find(|k| k.0.as_str() == "x").unwrap().1;
        let ymax = instr.iter().rev().find(|k| k.0.as_str() == "y").unwrap().1;

        let ans = (0..ymax)
            .map(|y| {
                (0..xmax)
                    .map(|x| if folded.contains(&(x, y)) { 'X' } else { ' ' })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");

        Answer::Str(ans)
    }
}

// Don't forget to set AOC_TOKEN
fn main() {
    let problem = aoc_problem!(year = 2021, day = 13);
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
    fn test_solution_a() {
        let solver = Solver {};
        let input = r"
            |6,10
            |0,14
            |9,10
            |0,3
            |10,4
            |4,11
            |6,0
            |6,12
            |4,1
            |0,13
            |10,12
            |3,4
            |3,0
            |8,4
            |1,10
            |2,14
            |8,10
            |9,0
            |
            |fold along y=7
            |fold along x=5
        "
        .trim()
        .strip_margin();

        let ans = solver.solve_a(input.lines());
        assert_eq!(ans, Answer::Num(17));

        let ans = solver.solve_b(input.lines());
        assert_eq!(
            ans,
            Answer::Str("XXXXX\nX   X\nX   X\nX   X\nXXXXX\n     \n     ".to_string())
        );
    }
}
