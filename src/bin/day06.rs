use adventofcode::{self as aoc, aoc_problem, Solution};
use std::str::Lines;

struct Solver;

impl Solver {
    fn count_fish(&self, input: Lines, days: usize) -> usize {
        let mut timers = [0; 9];

        input
            .flat_map(|line| line.split(',').map(|n| n.parse::<usize>().unwrap()))
            .for_each(|t| timers[t] += 1);

        for d in 0..days {
            timers[(d + 7) % timers.len()] += timers[d % timers.len()];
        }

        timers.iter().sum()
    }
}

impl Solution for Solver {
    type Output = usize;

    fn solve_a(&self, input: Lines) -> usize {
        self.count_fish(input, 80)
    }

    fn solve_b(&self, input: Lines) -> usize {
        self.count_fish(input, 256)
    }
}

// Don't forget to set AOC_TOKEN
fn main() {
    let problem = aoc_problem!(year = 2021, day = 6);
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
            |3,4,3,1,2
        "
        .trim()
        .strip_margin();

        let ans = solver.solve_a(input.lines());
        assert_eq!(ans, 5_934);

        let ans = solver.solve_b(input.lines());
        assert_eq!(ans, 26_984_457_539);
    }
}
