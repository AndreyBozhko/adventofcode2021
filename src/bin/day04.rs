use adventofcode::{self as aoc, aoc_problem, map, Solution};
use itertools::Itertools;
use std::collections::HashMap;
use std::str::Lines;

#[derive(Debug)]
struct Board {
    done: bool,
    marked: [[bool; 5]; 5],
    num2pos: HashMap<u32, (usize, usize)>,
}

impl Board {
    fn new(nums: &[u32]) -> Self {
        assert_eq!(nums.len(), 25);
        let mut positions = map![];
        for (i, &n) in nums.iter().enumerate() {
            positions.insert(n, (i / 5, i % 5));
        }
        Self {
            done: false,
            marked: [[false; 5]; 5],
            num2pos: positions,
        }
    }

    fn mark(&mut self, num: u32) -> Option<u32> {
        if self.done {
            return None;
        }
        if let Some(&(i, j)) = self.num2pos.get(&num) {
            self.marked[i][j] = true;

            let row_complete = self.marked[i].iter().all_equal();
            let col_complete = self.marked.iter().map(|el| el[j]).all_equal();

            if row_complete || col_complete {
                self.done = true;

                let score = self
                    .num2pos
                    .iter()
                    .filter_map(
                        |(&el, &(i, j))| {
                            if !self.marked[i][j] {
                                Some(el)
                            } else {
                                None
                            }
                        },
                    )
                    .sum();

                return Some(score);
            }
        }
        None
    }
}

struct Solver;

impl Solver {
    fn parse_input(&self, mut input: Lines) -> (Vec<u32>, Vec<Board>) {
        let sequence: Vec<u32> = input
            .next()
            .unwrap()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();

        let mut boards = vec![];

        while let Some(_) = input.next() {
            let rows: Vec<u32> = [
                input.next().unwrap(),
                input.next().unwrap(),
                input.next().unwrap(),
                input.next().unwrap(),
                input.next().unwrap(),
            ]
            .iter()
            .flat_map(|line| line.split_whitespace().map(|n| n.parse().unwrap()))
            .collect();

            boards.push(Board::new(&rows));
        }

        (sequence, boards)
    }
}

impl Solution for Solver {
    type Output = u32;

    fn solve_a(&self, input: Lines) -> u32 {
        let (sequence, mut boards) = self.parse_input(input);

        for drawn in sequence {
            for board in &mut boards {
                if let Some(score) = board.mark(drawn) {
                    return score * drawn;
                }
            }
        }
        unreachable!("Solution should exist");
    }

    fn solve_b(&self, input: Lines) -> u32 {
        let (sequence, mut boards) = self.parse_input(input);

        let mut last_score = 0;

        for drawn in sequence {
            for board in &mut boards {
                if let Some(score) = board.mark(drawn) {
                    last_score = score * drawn;
                }
            }
        }
        last_score
    }
}

// Don't forget to set AOC_TOKEN
fn main() {
    let problem = aoc_problem!(year = 2021, day = 4);
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
            |7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1
            |
            |22 13 17 11  0
            | 8  2 23  4 24
            |21  9 14 16  7
            | 6 10  3 18  5
            | 1 12 20 15 19
            |
            | 3 15  0  2 22
            | 9 18 13 17  5
            |19  8  7 25 23
            |20 11 10 24  4
            |14 21 16 12  6
            |
            |14 21 17 24  4
            |10 16 15  9 19
            |18  8 23 26 20
            |22 11 13  6  5
            | 2  0 12  3  7
        "
        .trim()
        .strip_margin();

        let ans = solver.solve_a(input.lines());
        assert_eq!(ans, 4512);

        let ans = solver.solve_b(input.lines());
        assert_eq!(ans, 1924);
    }
}
