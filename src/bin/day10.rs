use adventofcode::{self as aoc, aoc_problem, map, Solution};
use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::{HashMap, VecDeque};
use std::str::Lines;

lazy_static! {
    static ref PAIRS: HashMap<char, char> = map![
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
    ];
    static ref SCORES_1: HashMap<char, usize> = map![
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
    ];
    static ref SCORES_2: HashMap<char, usize> = map![
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
    ];
}

enum Type {
    Corrupted(char),
    Incomplete(String),
    Good,
}

impl Type {
    fn corrupted(self) -> Option<char> {
        match self {
            Type::Corrupted(c) => Some(c),
            _ => None,
        }
    }
    fn incomplete(self) -> Option<String> {
        match self {
            Type::Incomplete(s) => Some(s),
            _ => None,
        }
    }
}

trait Analyze {
    fn analyze(&self) -> Type;
}

impl Analyze for &str {
    fn analyze(&self) -> Type {
        let mut deque = VecDeque::new();
        for c in self.chars() {
            match c {
                '(' | '[' | '{' | '<' => {
                    deque.push_back(c);
                }
                ')' | ']' | '}' | '>' => match deque.pop_back().unwrap() {
                    b if PAIRS.get(&b) != Some(&c) => {
                        return Type::Corrupted(c);
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        if deque.is_empty() {
            return Type::Good;
        }

        let remaining: String = deque.iter().map(|c| PAIRS.get(c).unwrap()).collect();

        Type::Incomplete(remaining)
    }
}

struct Solver;

impl Solution for Solver {
    type Output = usize;

    fn solve_a(&self, input: Lines) -> usize {
        input
            .flat_map(|line| line.analyze().corrupted())
            .map(|c| SCORES_1.get(&c).copied().unwrap())
            .sum()
    }

    fn solve_b(&self, input: Lines) -> usize {
        let scores: Vec<_> = input
            .flat_map(|line| line.analyze().incomplete())
            .map(|el| {
                el.chars()
                    .zip(0..)
                    .map(|(c, pw)| SCORES_2.get(&c).unwrap() * 5_usize.pow(pw))
                    .sum()
            })
            .sorted()
            .collect();

        scores[(scores.len() - 1) / 2]
    }
}

// Don't forget to set AOC_TOKEN
fn main() {
    let problem = aoc_problem!(year = 2021, day = 10);
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
            - [({(<(())[]>[[{[]{<()<>>
            - [(()[<>])]({[<{<<[]>>(
            - {([(<{}[<>[]}>{[]{[(<()>
            - (((({<>}<{<{<>}{[]{[]{}
            - [[<[([]))<([[{}[[()]]]
            - [{[{({}]{}}([{[{{{}}([]
            - {<[[]]>}<{[{[{[]{()[[[]
            - [<(<(<(<{}))><([]([]()
            - <{([([[(<>()){}]>(<<{{
            - <{([{{}}[<[[[<>{}]]]>[]]
        "
        .trim()
        .strip_margin_of("- ");

        let ans = solver.solve_a(input.lines());
        assert_eq!(ans, 26397);

        let ans = solver.solve_b(input.lines());
        assert_eq!(ans, 288957);
    }
}
