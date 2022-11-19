use adventofcode::{self as aoc, aoc_problem, Solution};
use std::str::Lines;

struct Solver;

impl Solver {
    fn align_crabs<F>(&self, positions: Vec<i32>, dist_fn: F) -> i32
    where
        F: Fn(i32, i32) -> i32,
    {
        let (left, right) = (
            *positions.iter().min().unwrap(),
            *positions.iter().max().unwrap(),
        );

        (left..=right)
            .map(|pos| positions.iter().map(|&el| dist_fn(el, pos)).sum())
            .min()
            .unwrap()
    }
}

impl Solution for Solver {
    type Output = i32;

    fn solve_a(&self, input: Lines) -> i32 {
        let positions: Vec<i32> = input
            .flat_map(|line| line.split(',').map(|n| n.parse().unwrap()))
            .collect();

        self.align_crabs(positions, |x1, x2| (x1 - x2).abs())
    }

    fn solve_b(&self, input: Lines) -> i32 {
        let positions: Vec<i32> = input
            .flat_map(|line| line.split(',').map(|n| n.parse().unwrap()))
            .collect();

        self.align_crabs(positions, |x1, x2| {
            let dx = (x1 - x2).abs();
            dx * (dx + 1) / 2
        })
    }
}

// Don't forget to set AOC_TOKEN
fn main() {
    let problem = aoc_problem!(year = 2021, day = 7);
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
            |16,1,2,0,4,2,7,1,2,14
        "
        .trim()
        .strip_margin();

        let ans = solver.solve_a(input.lines());
        assert_eq!(ans, 37);

        let ans = solver.solve_b(input.lines());
        assert_eq!(ans, 168);
    }
}
