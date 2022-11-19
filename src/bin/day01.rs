use adventofcode::{self as aoc, aoc_problem, Solution};
use std::str::Lines;

struct Solver;

impl Solver {
    fn solve_for_window(&self, input: Lines, window_sz: usize) -> usize {
        let depths: Vec<i32> = input.map(|line| line.parse().unwrap()).collect();

        depths
            .windows(window_sz + 1)
            .filter(|&nums| nums[0] < nums[window_sz])
            .count()
    }
}

impl Solution for Solver {
    type Output = usize;

    fn solve_a(&self, input: Lines) -> usize {
        self.solve_for_window(input, 1)
    }

    fn solve_b(&self, input: Lines) -> usize {
        self.solve_for_window(input, 3)
    }
}

// Don't forget to set AOC_TOKEN
fn main() {
    let problem = aoc_problem!(year = 2021, day = 1);
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
            |199
            |200
            |208
            |210
            |200
            |207
            |240
            |269
            |260
            |263
        "
        .trim()
        .strip_margin();

        let ans = solver.solve_a(input.lines());
        assert_eq!(ans, 7);

        let ans = solver.solve_b(input.lines());
        assert_eq!(ans, 5);
    }
}
